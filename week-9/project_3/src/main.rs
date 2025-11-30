use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("ministers.csv")?;

    file.write_all("S/N,Name of Commissioner,Ministry,Geopolitical Zone\n".as_bytes())?;

    let records = vec![
        "1,Aigbogun Alamba Daudu,Internal Affairs,South West\n",
        "2,Murtala Afeez Bendu,Justice,North East\n",
        "3,Okorocha Calistus Ogbonna,Defense,South South\n",
        "4,Adewale Jimoh Akanbi,Power & Steel,South West\n",
        "5,Osazuwa Faith Etiyeye,Petroleum,South East\n",
    ];

    for record in records {
        file.write_all(record.as_bytes())?;
    }
    println!("File uploaded successfully");
    Ok(())
}
