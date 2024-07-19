use sqlx::PgPool;

struct Querier<T>(pub PgPool) {

}