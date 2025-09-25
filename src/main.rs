use postgres::{ Client, Error, NoTls};


fn main()-> Result<(), Error> {
     let mut client = Client::connect("host=localhost user=postgres",NoTls)?;

    for row in client.query("select id from public.equipe", &[])? {
       let all: i32 = row.get(0);


       println!("test : {}", all);
    }
    Ok(())
}

