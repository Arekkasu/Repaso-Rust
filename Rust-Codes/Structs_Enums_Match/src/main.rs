use ::std::io;
use core::option::Option;
/*
 * Estás implementando una bandeja de entrada de notificaciones.
 * Cada notificación puede ser de diferente tipo, y algunas de ellas
 * traen contenido adicional, como un mensaje.
 */

fn input() -> String {
    let mut input = String::new();
    loop {
        input.clear(); /* Para que siempre valor anterior se elimine */
        io::stdin()
            .read_line(&mut input)
            .expect("Error al ingresar el input");
        if !input.trim().is_empty() {
            break;
        }
        println!("No se puede ingresar un valor vacio!, Ingresa nuevamente el valor")
    }
    input
}

#[derive(Debug)]
struct MessageNotification {
    message: String,
    author: String,
}

impl MessageNotification {
    fn builder_message_notification() -> MessageNotification {
        println!("Ingresa el mensaje");
        let input_message = input();
        println!("Ingresa el autor");
        let input_author = input();
        MessageNotification {
            message: input_message,
            author: input_author,
        }
    }
}

#[derive(Debug)]
enum Notifications {
    Alert(String),
    Ping,
    Message(MessageNotification),
}

impl Notifications {
    fn see_notification(&self) {
        match self {
            Notifications::Alert(text) => println!("{}", text),
            Notifications::Message(m) => println!("{}, (Autor: {})", m.message, m.author),
            Notifications::Ping => println!("Ping ha sido lanzado"),
        }
    }
}

fn main() {
    loop {
        println!("Ingresa la notificación que deseas Lanzar (Message, Ping, Alert): ");
        /*  ESTO ES UN STRING VACIO */

        let mut notification: Option<Notifications> = None;
        let input_command = input();
        match input_command.trim() {
            "Message" => {
                notification = Some(Notifications::Message(
                    MessageNotification::builder_message_notification(),
                ))
            }
            "Ping" => notification = Some(Notifications::Ping),
            "Alert" => {
                let text_alert = input();
                notification = Some(Notifications::Alert(text_alert))
            }
            _ => println!("Opcion no existe"),
        }

        // ESTA SECCION DE CODIGO AL EJECUTARSE  SE ELEMINA DEL STACK LIBERANDO LA MEMORIA
        // ES UNA FORMA DE ASEGURAR LA INVOCACION DEL OPTION
        if let Some(notify) = &notification {
            notify.see_notification()
        }
        if input_command == "Salir" {
            break;
        }
    }
}
