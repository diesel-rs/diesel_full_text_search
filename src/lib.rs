#[macro_use]
extern crate diesel;

mod types {
    use diesel::sql_types::*;

    #[derive(Clone, Copy, SqlType)]
    #[diesel(postgres_type(oid = 3615, array_oid = 3645))]
    pub struct TsQuery;

    #[derive(Clone, Copy, SqlType)]
    #[diesel(postgres_type(oid = 3614, array_oid = 3643))]
    pub struct TsVector;
    pub type Tsvector = TsVector;

    pub trait TextOrNullableText {}

    impl TextOrNullableText for Text {}
    impl TextOrNullableText for Nullable<Text> {}

    #[derive(SqlType)]
    #[diesel(postgres_type(name = "regconfig"))]
    pub struct RegConfig;
}

pub mod configuration {
    use crate::RegConfig;

    use diesel::backend::Backend;
    use diesel::deserialize::{self, FromSql, FromSqlRow};
    use diesel::expression::{is_aggregate, ValidGrouping};
    use diesel::pg::{Pg, PgValue};
    use diesel::query_builder::{AstPass, QueryFragment, QueryId};
    use diesel::serialize::{self, Output, ToSql};
    use diesel::sql_types::Integer;
    use diesel::{AppearsOnTable, Expression, QueryResult, SelectableExpression};

    #[derive(Debug, PartialEq, Eq, diesel::expression::AsExpression, FromSqlRow)]
    #[diesel(sql_type = RegConfig)]
    pub struct TsConfiguration(pub u32);

    impl TsConfiguration {
        pub const SIMPLE: Self = Self(3748);
        pub const DANISH: Self = Self(12824);
        pub const DUTCH: Self = Self(12826);
        pub const ENGLISH: Self = Self(12828);
        pub const FINNISH: Self = Self(12830);
        pub const FRENCH: Self = Self(12832);
        pub const GERMAN: Self = Self(12834);
        pub const HUNGARIAN: Self = Self(12836);
        pub const ITALIAN: Self = Self(12838);
        pub const NORWEGIAN: Self = Self(12840);
        pub const PORTUGUESE: Self = Self(12842);
        pub const ROMANIAN: Self = Self(12844);
        pub const RUSSIAN: Self = Self(12846);
        pub const SPANISH: Self = Self(12848);
        pub const SWEDISH: Self = Self(12850);
        pub const TURKISH: Self = Self(12852);
    }

    impl FromSql<RegConfig, Pg> for TsConfiguration
    where
        i32: FromSql<Integer, Pg>,
    {
        fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
            <i32 as FromSql<Integer, Pg>>::from_sql(bytes).map(|oid| TsConfiguration(oid as u32))
        }
    }

    impl ToSql<RegConfig, Pg> for TsConfiguration
    where
        i32: ToSql<Integer, Pg>,
    {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
            <i32 as ToSql<Integer, Pg>>::to_sql(&(self.0 as i32), &mut out.reborrow())
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct TsConfigurationByName(pub &'static str);

    impl<DB> QueryFragment<DB> for TsConfigurationByName
    where
        DB: Backend,
    {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
            out.push_sql(&format!("'{}'", &self.0));
            Ok(())
        }
    }

    impl<GB> ValidGrouping<GB> for TsConfigurationByName {
        type IsAggregate = is_aggregate::Never;
    }

    impl QueryId for TsConfigurationByName {
        const HAS_STATIC_QUERY_ID: bool = false;

        type QueryId = ();
    }

    impl<QS> SelectableExpression<QS> for TsConfigurationByName where Self: Expression {}

    impl<QS> AppearsOnTable<QS> for TsConfigurationByName where Self: Expression {}

    impl Expression for TsConfigurationByName {
        type SqlType = RegConfig;
    }
}

#[allow(deprecated)]
mod functions {
    use diesel::sql_types::*;
    use types::*;

    sql_function!(fn length(x: TsVector) -> Integer);
    sql_function!(fn numnode(x: TsQuery) -> Integer);
    sql_function!(fn plainto_tsquery(x: Text) -> TsQuery);
    sql_function! {
        #[sql_name = "plainto_tsquery"]
        fn plainto_tsquery_with_search_config(config: RegConfig, querytext: Text) -> TsQuery;
    }
    sql_function!(fn querytree(x: TsQuery) -> Text);
    sql_function!(fn strip(x: TsVector) -> TsVector);
    sql_function!(fn to_tsquery(x: Text) -> TsQuery);
    sql_function! {
        #[sql_name = "to_tsquery"]
        fn to_tsquery_with_search_config(config: RegConfig, querytext: Text) -> TsQuery;
    }
    sql_function!(fn to_tsvector<T: TextOrNullableText + SingleValue>(x: T) -> TsVector);
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn to_tsvector_with_search_config<T: TextOrNullableText + SingleValue>(config: RegConfig, document_content: T) -> TsVector;
    }
    sql_function!(fn ts_headline(x: Text, y: TsQuery) -> Text);
    sql_function! {
        #[sql_name = "ts_headline"]
        fn ts_headline_with_search_config(config: RegConfig, x: Text, y: TsQuery) -> Text;
    }
    sql_function!(fn ts_rank(x: TsVector, y: TsQuery) -> Float);
    sql_function!(fn ts_rank_cd(x: TsVector, y: TsQuery) -> Float);
    sql_function! {
        #[sql_name = "ts_rank_cd"]
        fn ts_rank_cd_weighted(w: Array<Float>, x: TsVector, y: TsQuery) -> Float;
    }
    sql_function! {
        #[sql_name = "ts_rank_cd"]
        fn ts_rank_cd_normalized(x: TsVector, y: TsQuery, n: Integer) -> Float;
    }
    sql_function! {
        #[sql_name = "ts_rank_cd"]
        fn ts_rank_cd_weighted_normalized(w: Array<Float>, x: TsVector, y: TsQuery, n: Integer) -> Float;
    }
    sql_function!(fn phraseto_tsquery(x: Text) -> TsQuery);
    sql_function!(fn websearch_to_tsquery(x: Text) -> TsQuery);
    sql_function! {
        #[sql_name = "websearch_to_tsquery"]
        fn websearch_to_tsquery_with_search_config(config: RegConfig, x: Text) -> TsQuery;
    }
    sql_function!(fn setweight(x: TsVector, w: CChar) -> TsVector);
}

mod dsl {
    use diesel::expression::{AsExpression, Expression};
    use types::*;

    mod predicates {
        use diesel::pg::Pg;
        use types::*;

        diesel::infix_operator!(Matches, " @@ ", backend: Pg);
        diesel::infix_operator!(Concat, " || ", TsVector, backend: Pg);
        diesel::infix_operator!(And, " && ", TsQuery, backend: Pg);
        diesel::infix_operator!(Or, " || ", TsQuery, backend: Pg);
        diesel::infix_operator!(Contains, " @> ", backend: Pg);
        diesel::infix_operator!(ContainedBy, " <@ ", backend: Pg);
    }

    use self::predicates::*;

    pub trait TsVectorExtensions: Expression<SqlType = TsVector> + Sized {
        fn matches<T: AsExpression<TsQuery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<TsVector>>(self, other: T) -> Concat<Self, T::Expression> {
            Concat::new(self, other.as_expression())
        }
    }

    pub trait TsQueryExtensions: Expression<SqlType = TsQuery> + Sized {
        fn matches<T: AsExpression<TsVector>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn and<T: AsExpression<TsQuery>>(self, other: T) -> And<Self, T::Expression> {
            And::new(self, other.as_expression())
        }

        fn or<T: AsExpression<TsQuery>>(self, other: T) -> Or<Self, T::Expression> {
            Or::new(self, other.as_expression())
        }

        fn contains<T: AsExpression<TsQuery>>(self, other: T) -> Contains<Self, T::Expression> {
            Contains::new(self, other.as_expression())
        }

        fn contained_by<T: AsExpression<TsQuery>>(
            self,
            other: T,
        ) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType = TsVector>> TsVectorExtensions for T {}

    impl<T: Expression<SqlType = TsQuery>> TsQueryExtensions for T {}
}

pub use self::dsl::*;
pub use self::functions::*;
pub use self::types::*;
