use std::io;

fn main() {
    println!("Bienvenido a la búsqueda de criptomonedas!");
    loop {
        println!("Escribe el nombre de la criptomoneda que quieres buscar o escribe 'exit' para salir");
        // Mostrar lista de criptomonedas populares
        println!("----------------------------------------------------");
        println!("Ejemplo de solicitud: ✅bitcoin, ✅ethereum, ✅dogecoin, etc.");
        println!("puedes buscar con espacios, por ejemplo: ✅bitcoin cash");
        println!("----------------------------------------------------");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("⛔ Error al leer la entrada del usuario ⛔");

        let trimmed_lowercase = input.trim().to_lowercase();

        // Si la entrada contiene un espacio, reemplazarlo por un guión
        let replace_input = trimmed_lowercase.replace(" ", "-");

        if replace_input == "exit" {
            println!("¡Hasta luego! 🚀");
            break;
        } else {
            println!("⌛ Buscando información sobre {} ⌛", input.trim());

            // Realizar solicitud a la API
            let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", replace_input.trim());

            // Manejar errores al hacer la solicitud a la API
            let resp = match reqwest::blocking::get(&url) {
                Ok(resp) => resp,
                Err(err) => {
                    println!("⛔ Error al hacer la consulta: {} ⛔", err);
                    continue; // continuar con el siguiente ciclo
                }
            };

            // Procesar respuesta JSON
            let json_resp: serde_json::Value = match resp.json() {
                Ok(json_resp) => json_resp,
                Err(err) => {
                    println!("⛔ Error al procesar la respuesta: {} ⛔", err);
                    continue; // continuar con el siguiente ciclo
                }
            };

            // Obtener precio de la criptomoneda de la respuesta JSON
            let price = match json_resp[replace_input.trim()]["usd"].as_f64() {
                Some(price) => price,
                None => {
                    println!("⛔ No se pudo obtener el precio para {}, por favor intenta de nuevo ⛔", replace_input.trim());
                    continue; // continuar con el siguiente ciclo
                }
            };

            // Mostrar el precio de la criptomoneda
            println!(" ");
            println!("----------****************************--------------");
            println!("El precio de {} es ${} USD 💰", input.trim(), price);
            println!("----------****************************--------------");
            println!(" ");
        }
    }
}



