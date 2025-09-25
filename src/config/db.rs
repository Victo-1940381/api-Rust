use postgres::{Client};

fn test(){
  let mut client = Client::connect("host=localhost user=postgres dbname=apirust")?;

    for row in client.query("select * from public.equipe", &[])?{
       let all = row.get();


       println!(all);
    }
}
