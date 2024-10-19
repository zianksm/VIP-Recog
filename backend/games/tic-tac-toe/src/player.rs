use std::ops::Deref;

/// used as default, no player should have this
const DEFAULT_UUID: uuid::Uuid = uuid::uuid!("455dba63-dac9-4ea6-8291-fb2614482643");

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player(uuid::Uuid);

impl AsRef<uuid::Uuid> for Player {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for Player {
    fn default() -> Self {
        Self(DEFAULT_UUID.clone())
    }
}
impl Deref for Player {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Player {
    pub fn new() -> Self {
        let mut uuid;

        /// loop until we get uuid that's not default, will probably succeed in 1 try
        loop {
            uuid = uuid::Uuid::new_v4();

            match uuid.eq(&DEFAULT_UUID) {
                true => {
                    continue;
                }
                false => {
                    break;
                }
            }
        }

        Self(uuid)
    }
}
