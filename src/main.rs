use std::io;

fn main() {
    println!("Bienvenido a la búsqueda de criptomonedas!");
    loop {
        println!("Escribe el nombre de la criptomoneda que quieres buscar o escribe 'exit' para salir");

        // Mostrar lista de criptomonedas populares
        println!("----------------------------------------------------");
        println!("Ejemplo de solicitud: bitcoin, ethereum, dogecoin, etc.");
        println!("----------------------------------------------------");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada del usuario");

        let trimmed_input = input.trim().to_lowercase();

        if trimmed_input == "exit" {
            println!("¡Hasta luego!");
            break;
        } else {
            // Aquí va el código para buscar la criptomoneda
            println!("Buscando información sobre {}", trimmed_input);
            // Hacemos la consulta a una API externa para obtener el precio de la criptomoneda
            let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", input.trim());
            let resp = reqwest::blocking::get(&url).expect("Error al hacer la consulta");
            let json_resp: serde_json::Value = resp.json().expect("Error al procesar la respuesta");
            let price = json_resp[input.trim()]["usd"].as_f64().expect("No se pudo obtener el precio");
            
            println!("----------****************************--------------");
            println!("El precio de {} es ${}", input.trim(), price);
            println!("----------****************************--------------");
            println!(" ");
            println!(" ");

        }
    }
}


