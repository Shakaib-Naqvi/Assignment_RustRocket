#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
extern crate text_io;
use rocket::response::{Flash, Redirect };
use rocket::request::Form;
use rocket::response::content;
use rocket::response::content::Html;


// Assignment done using rocket framework.


#[get("/number/<n>")]
fn index(n:i32)->String{
    let add = n + 50; //it will add 50 to the given parameter
    format!("The number you entered is {} and by adding 50 the number wil be {}.",n,add)
}
// By entering localhost:8000/number/45 in browser it will return The number you entered is 45 and by adding 50 the number wil be 95.

fn main(){
    rocket::ignite().mount("/",routes![index]).launch();
}




// Bonus Task: (optional)
// Send a POST request with html form to the api you just made and show the response



// #[get("/?<n>")]
// fn html(n:i32)-> content::Html<String>{
    //     let add = n + 50;
    //     let show = format!(r#"
// <!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta charset="UTF-8">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>Assignment</title>
// </head>
// <body>
//     <div style="background-color: red;margin-top: 5%;padding: 200px;">
//     <div align="center" style="color: white; font-size: 50px;">
//         <h1>Assignment</h1>
//     </div>
//     <div align="center" style="color: white; font-size: 40px;">
//         <h3>Topic:</h3>
//     </div>
//     <div align= "center"; style="color: white; font-size: 30px;font-style: italic;">
//         <h3>Create an api endpoint which accepts a query parameter and in response, adds 50 the parameter and return it.</h3>
//     </div>

//     <form action="/" method="get">
//         <div align="center" style="background-color: grey; padding: 20% 20% 20% 20%;">
//             <input type="text" placeholder="Enter a number" style="font-size: 50px;width:100%;"> 
//             <br>
//             <br>
//             <button type="submit" style="font-size: 50px; width: 100%; color: white;background-color: red;;">Submit</button>
//             <h4 style="color:white;font-size:50px">The number you entered is {} and by adding 50 the number will {}</h4>
//         </div>
//     </form>
// </div>
// </body>
// </html>
//     "#,n,add);
//     content::Html(show)
// }
// #[get("/")]
// fn index_0()->content::Html<&'static str>{
//     content::Html(r#"
//     <!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta charset="UTF-8">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>Assignment</title>
// </head>
// <body>
//     <div style="background-color: red;margin-top: 5%;padding: 200px;">
//     <div align="center" style="color: white; font-size: 50px;">
//         <h1>Assignment</h1>
//     </div>
//     <div align="center" style="color: white; font-size: 40px;">
//         <h3>Topic:</h3>
//     </div>
//     <div align= "center"; style="color: white; font-size: 30px;font-style: italic;">
//         <h3>Create an api endpoint which accepts a query parameter and in response, adds 50 the parameter and return it.</h3>
//     </div>

//     <form action="/" method="get">
//         <div align="center" style="background-color: grey; padding: 20% 20% 20% 20%;">
//             <input type="text" placeholder="Enter a number" style="font-size: 50px;width:100%;" name="n"> 
//             <br>
//             <br>
//             <button type="submit" style="font-size: 50px; width: 100%; color: white;background-color: red;;">Submit</button>
//         </div>
//     </form>
// </div>
// </body>
// </html>
//     "#)
// }

// fn main(){
//     rocket::ignite().mount("/", routes![index_0,html]).launch();
// }

