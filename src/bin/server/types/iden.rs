use sea_query::Iden;
#[derive(Iden)]
pub enum Logins {
    Table,
    UserUuid,
    Email,
    Password,
}
