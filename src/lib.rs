#[macro_use]
extern crate diesel;

mod types {
    use std::io::Write;

    use diesel::backend::Backend;
    use diesel::deserialize::{self, FromSql};
    use diesel::serialize::{self, Output};
    use diesel::sql_types::Integer;
    use diesel::types::ToSql;

    #[allow(deprecated)]
    use diesel::SqlType;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3615", array_oid = "3645")]
    pub struct TsQuery;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3614", array_oid = "3643")]
    pub struct TsVector;
    pub type Tsvector = TsVector;

    pub const CONFIGURATION_SIMPLE: u32 = 3748;
    pub const CONFIGURATION_DANISH: u32 = 12824;
    pub const CONFIGURATION_DUTCH: u32 = 12826;
    pub const CONFIGURATION_ENGLISH: u32 = 12828;
    pub const CONFIGURATION_FINNISH: u32 = 12830;
    pub const CONFIGURATION_FRENCH: u32 = 12832;
    pub const CONFIGURATION_GERMAN: u32 = 12834;
    pub const CONFIGURATION_HUNGARIAN: u32 = 12836;
    pub const CONFIGURATION_ITALIAN: u32 = 12838;
    pub const CONFIGURATION_NORWEGIAN: u32 = 12840;
    pub const CONFIGURATION_PORTUGUESE: u32 = 12842;
    pub const CONFIGURATION_ROMANIAN: u32 = 12844;
    pub const CONFIGURATION_RUSSIAN: u32 = 12846;
    pub const CONFIGURATION_SPANISH: u32 = 12848;
    pub const CONFIGURATION_SWEDISH: u32 = 12850;
    pub const CONFIGURATION_TURKISH: u32 = 12852;

    #[derive(SqlType)]
    #[postgres(type_name = "regconfig")]
    pub struct Regconfig;

    #[derive(Debug, PartialEq, AsExpression)]
    #[sql_type = "Regconfig"]
    pub struct TsConfiguration(u32);

    impl<DB> FromSql<Regconfig, DB> for TsConfiguration
    where
        DB: Backend,
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            <i32 as FromSql<Integer, DB>>::from_sql(bytes).map(|oid| TsConfiguration(oid as u32))
        }
    }

    impl<DB> ToSql<Regconfig, DB> for TsConfiguration
    where
        DB: Backend,
        i32: ToSql<Integer, DB>,
    {
        fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
            <i32 as ToSql<Integer, DB>>::to_sql(&(*&self.0 as i32), out)
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
    sql_function!(fn to_tsvector(x: Text) -> TsVector);
    sql_function!(fn nullableto_tsvector(x: Nullable<Text>) -> TsVector);
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn to_tsvector_with_search_config(config: Regconfig, document_content: Text) -> TsVector;
    }
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn nullableto_tsvector_with_search_config(config: Regconfig, document_content: Nullable<Text>) -> TsVector;
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

        diesel_infix_operator!(Matches, " @@ ", backend: Pg);
        diesel_infix_operator!(Concat, " || ", TsVector, backend: Pg);
        diesel_infix_operator!(And, " && ", TsQuery, backend: Pg);
        diesel_infix_operator!(Or, " || ", TsQuery, backend: Pg);
        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
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
