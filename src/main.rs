    use postgres::{Client, NoTls, Error};

    fn main() -> Result<(), Error> {
        let mut client = Client::connect("host=localhost user=postgres dbname=apirust ", NoTls)?;

        for row in client.query("SELECT id, nom FROM equipe", &[])? {
            let id: i32 = row.get(0);
            let nom: String = row.get(1);
            println!("Found equipe: {} (ID: {})", nom, id);
        }

        Ok(())
    }