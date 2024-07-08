# PREX Core Challenge

## Mariano Massollo

### Julio 2024

---

## **Planteo del problema**
>
> Realizar un mini procesador de pagos con la capacidad de llevar el saldo de los clientes en memoria y persistirlos a demanda en un archivo.
> Se deberá implementar un micro servicio (usando Rust v1.71 o superior) que exponga una API REST, a través de la cual pueda llevar un registro del saldo de sus clientes.
>> Para mayor información referirse al archivo adjuntado PrexCPRE_Challenge.pdf
---

## **Entregables**
>
> Código fuente.
> Colección de Postman para la prueba de la API.
> Documentación.
---

## **Éste proyecto confía en:**
>
> actix-web
>> Versión: "4.8.0"
---
> serde
>> Versión: "1.0.204"
---
> tokio
>> Versión: "1.38.0"
---
> chrono
>> Versión: "0.4.38"
---

## **API Endpoints**

**POST **new_client****
> Creación de un nuevo registro de cliente.
> Parámetros (JSON):
>>{
>> "client_name": "Juan Perez",
>> "birth_date": "2000-01-20",
>> "document_number": "123456789",
>> "country": "AR"
>>}
> Retorno (HTTP Response):
>> "New client created with ID {}"

**POST **new_credit_transaction****
> Genera un incremento en el balance del cliente
> Parámetros (JSON):
>>{
>> "client_id": 1,
>> "credit_amount": 123.45,
>>}
> Retorno Success (HTTP Response):
>> "Client ID {} new balance {}"
> Retorno Failure (HTTP Response):
>> "Client ID {} NOT found"

**POST **new_debit_transaction****
> Genera un debito en el balance del cliente
> Parámetros (JSON):
>>{
>> "client_id": 1,
>> "debit_amount": 123.45,
>>}
> Retorno Success (HTTP Response):
>> "Client ID {} new balance {}"
> Retorno Failure (HTTP Response):
>> "Client ID {} NOT found"

**POST **store_balances****
> Consolida la información de los balances de todos los clientes.
> Todos los balances quedan en 0 al ejecutarse.
> No recibe Parámetros.

**GET **client_balance****
> Retorna el balance actual de un cliente.
> Parámetros (URL Param):
>> {user_id}
>> "/client_balance/1"
> Retorno (JSON):
>>{
>> "client_id": 1,
>> "balance": 123.45,
>>}
