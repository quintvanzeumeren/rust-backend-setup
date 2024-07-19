use std::marker::PhantomData;
use sqlx::PgPool;
use domain::permission::permission::Permission;

pub struct Querier<P: Permission> {
    pub db: PgPool,
    phantom_data: PhantomData<P>
}

impl <P: Permission> Querier<P> {

    pub fn new(db: PgPool) -> Querier<P> {
        Querier {
            db,
            phantom_data: PhantomData::default()
        }
    }
}



