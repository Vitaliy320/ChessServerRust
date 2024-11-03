use std::error::Error;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::types::{ToSql, FromSql, Type, IsNull, to_sql_checked};
use tokio_postgres::types::private::BytesMut;
use crate::game_status::GameStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameEndCondition {
    None,
    WhiteCheckmatedBlack,
    BlackCheckmatedWhite,
    WhiteResigned,
    BlackResigned,
    WhiteWonOnTime,
    BlackWonOnTime,
    Draw,
    Stalemate,
}

impl GameEndCondition {
    pub fn to_string(&self) -> String {
        match self {
            GameEndCondition::None => "None".to_string(),
            GameEndCondition::WhiteCheckmatedBlack => "WhiteCheckmatedBlack".to_string(),
            GameEndCondition::BlackCheckmatedWhite => "BlackCheckmatedWhite".to_string(),
            GameEndCondition::WhiteResigned => "WhiteResigned".to_string(),
            GameEndCondition::BlackResigned => "BlackResigned".to_string(),
            GameEndCondition::WhiteWonOnTime => "WhiteWonOnTime".to_string(),
            GameEndCondition::BlackWonOnTime => "BlackWonOnTime".to_string(),
            GameEndCondition::Draw => "Draw".to_string(),
            GameEndCondition::Stalemate => "Stalemate".to_string(),
        }
    }
}

impl ToSql for GameEndCondition {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        let condition_str = match *self {
            GameEndCondition::None => "None",
            GameEndCondition::WhiteCheckmatedBlack => "WhiteCheckmatedBlack",
            GameEndCondition::BlackCheckmatedWhite => "BlackCheckmatedWhite",
            GameEndCondition::WhiteResigned => "WhiteResigned",
            GameEndCondition::BlackResigned => "BlackResigned",
            GameEndCondition::WhiteWonOnTime => "WhiteWonOnTime",
            GameEndCondition::BlackWonOnTime => "BlackWonOnTime",
            GameEndCondition::Draw => "Draw",
            GameEndCondition::Stalemate => "Stalemate",
        };
        out.extend_from_slice(condition_str.as_bytes());
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

impl<'a> FromSql<'a> for GameEndCondition {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let status_str = std::str::from_utf8(raw).unwrap();
        match status_str {
            "None" => Ok(GameEndCondition::None),
            "WhiteCheckmatedBlack" => Ok(GameEndCondition::WhiteCheckmatedBlack),
            "BlackCheckmatedWhite" => Ok(GameEndCondition::BlackCheckmatedWhite),
            "WhiteResigned" => Ok(GameEndCondition::WhiteResigned),
            "BlackResigned" => Ok(GameEndCondition::BlackResigned),
            "WhiteWonOnTime" => Ok(GameEndCondition::WhiteWonOnTime),
            "BlackWonOnTime" => Ok(GameEndCondition::BlackWonOnTime),
            "Draw" => Ok(GameEndCondition::Draw),
            "Stalemate" => Ok(GameEndCondition::Stalemate),
            _ => Err("Unknown game status".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR || ty == &Type::TEXT
    }
}