use std::error::Error;
use tokio_postgres::types::{ToSql, FromSql, Type, IsNull, to_sql_checked};
use tokio_postgres::types::private::BytesMut;

#[derive(Clone, Debug)]
pub enum GameStatus {
    AwaitingOpponent,
    Ongoing,
    Finished,
}


impl ToSql for GameStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        let status_str = match *self {
            GameStatus::AwaitingOpponent => "AwaitingOpponent",
            GameStatus::Ongoing => "Ongoing",
            GameStatus::Finished => "Finished",
        };
        out.extend_from_slice(status_str.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        ty == &Type::VARCHAR || ty == &Type::TEXT
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for GameStatus {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let status_str = std::str::from_utf8(raw).unwrap();
        match status_str {
            "AwaitingOpponent" => Ok(GameStatus::AwaitingOpponent),
            "Ongoing" => Ok(GameStatus::Ongoing),
            "Finished" => Ok(GameStatus::Finished),
            _ => Err("Unknown game status".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR || ty == &Type::TEXT
    }
}