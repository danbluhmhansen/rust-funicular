use crate::migrations::Migration;
use pgrx::prelude::*;

struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        if !Spi::get_one_with_args::<bool>(
            r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                "230603095553_init".into_datum(),
            )],
        )
        .is_ok_and(|o| !o.is_some_and(|b| !b))
        {
            Spi::run(include_str!("up.sql"))?;
        }
        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        if Spi::get_one_with_args::<bool>(
            r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                "230603095553_init".into_datum(),
            )],
        )
        .is_ok_and(|o| o.is_some_and(|b| b))
        {
            Spi::run(include_str!("down.sql"))?;
        }
        Ok(())
    }
}

#[pg_extern]
pub fn _230603095553_init_up() -> Result<(), spi::Error> {
    _230603095553Init::up()
}

#[pg_extern]
pub fn _230603095553_init_down() -> Result<(), spi::Error> {
    _230603095553Init::down()
}