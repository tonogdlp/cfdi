
## License

[MIT](https://choosealicense.com/licenses/mit/)





 # CFDI.rs

 Crate para Deserializar un XML de un CFDI en structs nativos en Rust.


 ## Comprobante

 El struct principal es [`Comprobante`]. Al igual que en un xml de un CFDI, es el nodo de origen.

 Tambien hay structs para otros subnodos del xml, incluyendo sus atributos. Estos incluyen
 ['Emisor'], ['Receptor'] y ['TimbreFiscalDigital']

 Este crate por el momento solo contempla la estructura más básica de un CFDI.
 No se intenta generar un modelo completo del estandar de los CFDI 4.0+
 Hasta esta versión, solo se implementa el siguient modelo:

 ```markdown
|-Comprobante
     |-- Emisor
          |**-> rfc
          |**-> nombre
          |**-> regimen_fiscal
     |-- Receptor
          |**-> rfc
          |**-> nombre
          |**-> regimen_fiscal
          |**-> uso_cfdi
     |-- Conceptos
     |-- Complemento (opcional) - Incluye TimbreFiscalDigital
     |**-> total
     |**-> subtotal
     |**-> fecha
     |**-> forma_de_pago
     |**-> descuento
     |**-> tipo_comprobante
     |**-> forma_de_pago


 |-- Representan subnodos
 |**-> representan atributos del nodo
 ```


 Sin embargo, cualquier persona que necesite más detalle puede revisar el codigo fuente y
 de ahi basarse para agregar los subnodos que necesite. Realmente no es dificil, pero es
 un poco tedioso. Si agregan subnodos les agradecería hicieran un Pull Request.





 #### Ejemplo:
 Usando [`Comprobante`]:
 ```rust
 use cfdi::{DatosPrincipales, get_datos_principales};

 let path = ("path/to/file.xml");
 let xml_string = std::fs::read_to_string(&path).unwrap();


 if let Ok(parsed) = parse_cfdi(&xml_string) {

         println!("Emisor: {}, RFC: {}", parsed.emisor.nombre, parsed.emisor.rfc);
         println!("Receptor: {}, RFC: {}", parsed.receptor.nombre, parsed.receptor.rfc);
         println!("Subtotal:  {}", parsed.subtotal);
         println!("Total:  {}", parsed.total);
         println!("Fecha Factura:  {}", parsed.fecha);
         println!("UUID:  {}", parsed.complemento);
         println!("Fecha Timbrado:  {}", parsed.fecha_timbrado);


         // Otros datos no incluidos en [`DatosPrincipales`]:
         match &parsed.complemento {
             Some(c) => c.timbre_fiscal_digital
                 .clone()
                 .map(|tfd| println!("Certificado SAT: {}", tfd.no_certificado_sat),
                 None => None,
        }
 }
```


 ## Datos Principales
 [`DatosPrincipales`] es un struct que facilita recopilar en 1 solo nivel los principales
 atributos del cfdi. Asume que los datos se encuntran correctamente definidos en el cfdi.
 Además, no incluye otros datos que a veces ocupan mucho espacio, o simplemente no conviene
 estar manejando (ej. Sello, certificado, etc.)



 ```rust
 use cfdi::{DatosPrincipales, get_datos_principales};

 let path = ("path/to/file.xml");
 let xml_string = std::fs::read_to_string(&path).unwrap();

 if let Ok(parsed) = parse_cfdi(&xml_string) {
         let datos: DatosPrincipales = parsed.get_datos_principales();

         println!("Emisor: {}, RFC: {}", datos.emisor_nombre, datos.emisor_rfc);
         println!("Receptor: {}, RFC: {}", datos.receptor_nombre, datos.receptor_rfc);
         println!("Subtotal:  {}", datos.subtotal);
         println!("Total:  {}", datos.total);
         println!("Fecha Factura:  {}", datos.fecha);
         println!("UUID:  {}", datos.get_uuid());
         println!("Fecha Timbrado:  {}", datos.get_fecha_timbrado());

 }
 ```
