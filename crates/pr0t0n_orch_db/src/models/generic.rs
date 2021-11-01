use std::convert::{TryFrom, TryInto};

/// Generic queries that are required for all entities.
use diesel::{
    associations::HasTable,
    dsl::{Find, Limit},
    pg::Pg,
    prelude::*,
    query_builder::{
        AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget, QueryFragment,
    },
    query_dsl::{
        methods::{ExecuteDsl, FindDsl, LimitDsl, LoadQuery},
        RunQueryDsl,
    },
};

use crate::Error;

/// Allows finding 9by primary key) for this entity.
pub trait DbFind: Sized {
    type Table: Table + HasTable<Table = Self::Table> + LoadQuery<PgConnection, Self>;

    /// Query by ID.
    fn find<PK>(conn: &PgConnection, id: PK) -> Result<Self, diesel::result::Error>
    where
        Self::Table: FindDsl<PK>,
        Find<Self::Table, PK>: RunQueryDsl<PgConnection> + LimitDsl,
        Limit<Find<Self::Table, PK>>: LoadQuery<PgConnection, Self>,
    {
        diesel::QueryDsl::find(Self::Table::table(), id).first(conn)
    }

    /// Find all.
    fn find_all(connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        Self::Table::table().load::<Self>(connection)
    }
}

/// Allows mapped insertion.
pub trait DbMappedFind: Sized {
    type Queryable: TryInto<Self, Error = Error>;
    type Table: Table + HasTable<Table = Self::Table> + LoadQuery<PgConnection, Self::Queryable>;

    /// Query by ID.
    fn find<PK>(conn: &PgConnection, id: PK) -> Result<Self, Error>
    where
        Self::Table: FindDsl<PK>,
        Find<Self::Table, PK>: RunQueryDsl<PgConnection> + LimitDsl,
        Limit<Find<Self::Table, PK>>: LoadQuery<PgConnection, Self::Queryable>,
    {
        let result: Self::Queryable =
            diesel::QueryDsl::find(Self::Table::table(), id).first(conn)?;
        result.try_into()
    }

    /// Find all.
    fn find_all(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        Self::Table::table()
            .load::<Self::Queryable>(connection)?
            .into_iter()
            .map(|e| e.try_into())
            .collect()
    }
}

/// Allows insertion for this entity.
pub trait DbInsert: Sized {
    type Table: Table + HasTable<Table = Self::Table>;
    type Return;

    /// Insert a value into the table.
    fn insert<'a>(&'a self, conn: &PgConnection) -> Result<Self::Return, diesel::result::Error>
    where
        &'a Self: Insertable<Self::Table>,
        InsertStatement<Self::Table, <&'a Self as Insertable<Self::Table>>::Values>:
            RunQueryDsl<PgConnection> + LoadQuery<PgConnection, Self::Return>,
    {
        diesel::insert_into(Self::Table::table())
            .values(self)
            .get_result(conn)
    }
}

/// Allows mapped insertion for this entity.
pub trait DbMappedInsert<'a>: Sized + 'a {
    type Table: Table + HasTable<Table = Self::Table>;
    type Return;
    type Insertable: TryFrom<&'a Self, Error = Error> + Insertable<Self::Table>;
    type MappedReturn: TryFrom<Self::Return, Error = Error>;

    /// Insert a value into the table.
    fn insert(&'a self, conn: &PgConnection) -> Result<Self::MappedReturn, Error>
    where
        InsertStatement<Self::Table, <Self::Insertable as Insertable<Self::Table>>::Values>:
            RunQueryDsl<PgConnection> + LoadQuery<PgConnection, Self::Return>,
    {
        let result: Self::Return = diesel::insert_into(Self::Table::table())
            .values(Self::Insertable::try_from(self)?)
            .get_result(conn)?;
        result.try_into()
    }
}

type DeleteFindStatement<F> =
    DeleteStatement<<F as HasTable>::Table, <F as IntoUpdateTarget>::WhereClause>;

/// Allows deletion on this entity.
pub trait DbDelete {
    type Table: Table + HasTable<Table = Self::Table>;

    fn delete<PK>(conn: &PgConnection, id: PK) -> Result<usize, diesel::result::Error>
    where
        Self::Table: FindDsl<PK>,
        Find<Self::Table, PK>: IntoUpdateTarget,
        DeleteFindStatement<Find<Self::Table, PK>>: ExecuteDsl<PgConnection>,
    {
        let find = diesel::QueryDsl::find(Self::Table::table(), id);
        diesel::delete(find).execute(conn)
    }
}

/// Allows updating an entity.
pub trait DbUpdate: Sized {
    type Table: Table + IntoUpdateTarget + HasTable<Table = Self::Table>;

    /// Updates an existing entry in the database.
    fn update<'a>(&'a self, conn: &PgConnection) -> Result<usize, diesel::result::Error>
    where
        &'a Self: AsChangeset<Target = Self::Table>,
        <Self::Table as QuerySource>::FromClause:
            QueryFragment<<PgConnection as Connection>::Backend>,
        <Self::Table as IntoUpdateTarget>::WhereClause: QueryFragment<Pg>,
        <&'a Self as AsChangeset>::Changeset: QueryFragment<Pg>,
    {
        let update = diesel::update(Self::Table::table());
        update.set(self).execute(conn)
    }
}

/// Allows updating an entity.
pub trait DbMappedUpdate<'a>: Sized + 'a {
    type Table: Table + IntoUpdateTarget + HasTable<Table = Self::Table>;
    type Insertable: TryFrom<&'a Self, Error = Error>;

    /// Updates an existing entry in the database.
    fn update(&'a self, conn: &PgConnection) -> Result<usize, Error>
    where
        Self::Insertable: AsChangeset<Target = Self::Table>,
        <Self::Table as QuerySource>::FromClause:
            QueryFragment<<PgConnection as Connection>::Backend>,
        <Self::Table as IntoUpdateTarget>::WhereClause: QueryFragment<Pg>,
        <Self::Insertable as AsChangeset>::Changeset: QueryFragment<Pg>,
    {
        let insertable = Self::Insertable::try_from(self)?;
        let update = diesel::update(Self::Table::table());
        let count = update.set(insertable).execute(conn)?;
        Ok(count)
    }
}
