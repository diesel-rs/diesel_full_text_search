#[macro_use] extern crate diesel;

mod types {
    #[allow(deprecated)]
    use diesel::SqlType;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3615", array_oid = "3645")]
    pub struct TsQuery;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3614", array_oid = "3643")]
    pub struct TsVector;
    pub type Tsvector = TsVector;

    #[derive(SqlType)]
    #[postgres(type_name = "regconfig")]
    pub struct Regconfig;
}

#[allow(deprecated)]
mod functions {
    use types::*;
    use diesel::sql_types::*;

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
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn to_tsvector_with_search_config(config: Regconfig, document_content: Text) -> TsVector;
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
    use types::*;
    use diesel::expression::{Expression, AsExpression};
    use diesel::expression::grouped::Grouped;

    mod predicates {
        use types::*;
        use diesel::pg::Pg;

        diesel_infix_operator!(Matches, " @@ ", backend: Pg);
        diesel_infix_operator!(Concat, " || ", TsVector, backend: Pg);
        diesel_infix_operator!(And, " && ", TsQuery, backend: Pg);
        diesel_infix_operator!(Or, " || ", TsQuery, backend: Pg);
        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
        diesel_prefix_operator!(Negate, " !! ", TsQuery, backend: Pg);
    }

    use self::predicates::*;

    pub type Concat<T, U> = Grouped<predicates::Concat<T, U>>;

    pub trait TsVectorExtensions: Expression<SqlType=TsVector> + Sized {
        fn matches<T: AsExpression<TsQuery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<TsVector>>(self, other: T) -> Concat<Self, T::Expression> {
            Grouped(predicates::Concat::new(self, other.as_expression()))
        }
    }

    pub trait TsQueryExtensions: Expression<SqlType=TsQuery> + Sized {
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

        fn contained_by<T: AsExpression<TsQuery>>(self, other: T) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType=TsVector>> TsVectorExtensions for T {
    }

    impl<T: Expression<SqlType=TsQuery>> TsQueryExtensions for T {
    }

    pub fn negate<T: AsExpression<TsQuery>>(expr: T) -> Negate<Grouped<T::Expression>> {
        Negate::new(Grouped(expr.as_expression()))
    }
}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;
