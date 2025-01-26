use derive_new::new;

#[derive(Clone, Debug, PartialEq, new)]
pub struct UserID(#[new(value = "uuid::Uuid::now_v7()")] uuid::Uuid);

impl UserID {
    pub fn inner(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl std::str::FromStr for UserID {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(input)
            .map(Self)
            .map_err(|e| anyhow::anyhow!("invalid user id: {}", e))
    }
}

impl From<uuid::Uuid> for UserID {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl sqlx::Type<sqlx::Postgres> for UserID {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("uuid")
    }
}

impl<'a> sqlx::Encode<'a, sqlx::Postgres> for UserID {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'a>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        self.inner().encode_by_ref(buf)
    }
}

impl<'a> sqlx::Decode<'a, sqlx::Postgres> for UserID {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'a>
    ) -> Result<Self, sqlx::error::BoxDynError> {
        uuid::Uuid::decode(value).map(UserID)
    }
}
