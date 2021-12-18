#[macro_use]
extern crate diesel;

mod types {
    use diesel::sql_types::*;

    #[allow(deprecated)]
    use diesel::SqlType;

    #[derive(Clone, Copy, SqlType)]
    #[diesel(postgres_type(oid = 3615, array_oid = 3645))]
    pub struct TsQuery;

    #[derive(Clone, Copy, SqlType)]
    #[diesel(postgres_type(oid = 3614, array_oid = 3643))]
    pub struct TsVector;
    pub type Tsvector = TsVector;

    pub trait TextOrNullableText: SingleValue {}

    impl TextOrNullableText for Text {}
    impl TextOrNullableText for Nullable<Text> {}

    #[derive(SqlType)]
    #[diesel(postgres_type(name = "regconfig"))]
    pub struct Regconfig;
}

pub mod configuration {
    use crate::Regconfig;

    use diesel::backend::{Backend, RawValue};
    use diesel::deserialize::{self, FromSql};
    use diesel::pg::Pg;
    use diesel::serialize::{self, Output, ToSql};
    use diesel::sql_types::Integer;

    #[derive(Debug, PartialEq, AsExpression)]
    #[diesel(sql_type = Regconfig)]
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

    impl<DB> FromSql<Regconfig, DB> for TsConfiguration
    where
        DB: Backend,
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(bytes: RawValue<DB>) -> deserialize::Result<Self> {
            <i32 as FromSql<Integer, DB>>::from_sql(bytes).map(|oid| TsConfiguration(oid as u32))
        }
    }

    impl ToSql<Regconfig, Pg> for TsConfiguration {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
            <i32 as ToSql<Integer, Pg>>::to_sql(&(self.0 as i32), &mut out.reborrow())
        }
    }
}

#[allow(deprecated)]
mod functions {
    use diesel::sql_types::*;
    use types::*;

    sql_function!(fn length(x: TsVector) -> Integer);
    sql_function!(fn numnode(x: TsQuery) -> Integer);
    sql_function!(fn plainto_tsquery(x: Text) -> TsQuery);
    sql_function!(fn querytree(x: TsQuery) -> Text);
    sql_function!(fn strip(x: TsVector) -> TsVector);
    sql_function!(fn to_tsquery(x: Text) -> TsQuery);
    sql_function! {
        #[sql_name = "to_tsquery"]
        fn to_tsquery_with_search_config(config: Regconfig, querytext: Text) -> TsQuery;
    }
    sql_function!(fn to_tsvector<T: TextOrNullableText>(x: T) -> TsVector);
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn to_tsvector_with_search_config<T: TextOrNullableText>(config: Regconfig, document_content: T) -> TsVector;
    }
    sql_function!(fn ts_headline(x: Text, y: TsQuery) -> Text);
    sql_function!(fn ts_rank(x: TsVector, y: TsQuery) -> Float);
    sql_function!(fn ts_rank_cd(x: TsVector, y: TsQuery) -> Float);
    sql_function! {
        #[sql_name = "ts_rank_cd"]
        fn ts_rank_cd_weighted(w: Array<Float>, x: TsVector, y: TsQuery) -> Float;
    }
    sql_function!(fn phraseto_tsquery(x: Text) -> TsQuery);
    sql_function!(fn websearch_to_tsquery(x: Text) -> TsQuery);
}

mod dsl {
    use diesel::expression::grouped::Grouped;
    use diesel::expression::{AsExpression, Expression};
    use types::*;

    mod predicates {
        use diesel::pg::Pg;
        use types::*;

        infix_operator!(Matches, " @@ ", backend: Pg);
        infix_operator!(Concat, " || ", TsVector, backend: Pg);
        infix_operator!(And, " && ", TsQuery, backend: Pg);
        infix_operator!(Or, " || ", TsQuery, backend: Pg);
        infix_operator!(Contains, " @> ", backend: Pg);
        infix_operator!(ContainedBy, " <@ ", backend: Pg);
    }

    use self::predicates::*;

    pub type Concat<T, U> = Grouped<predicates::Concat<T, U>>;

    pub trait TsVectorExtensions: Expression<SqlType = TsVector> + Sized {
        fn matches<T: AsExpression<TsQuery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<TsVector>>(self, other: T) -> Concat<Self, T::Expression> {
            Grouped(predicates::Concat::new(self, other.as_expression()))
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
