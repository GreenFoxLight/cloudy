#[macro_use]
extern crate mysql;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    let pool = mysql::Pool::new("mysql://root@localhost:3306").unwrap();

    // Create a temporary table
    pool.prep_exec(r"CREATE TEMPORARY TABLE tmp.payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )", ()).unwrap();

    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    for mut stmt in pool.prepare(r"INSERT INTO tmp.payment (customer_id, amount, account_name) VALUES (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            // execute takes ownership of params, so we pass account name by reference,
            // Unwrap each result to make sure no error happened.
            stmt.execute(params!{
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }).unwrap();
        }
    }

    // Select payments from database
    let selected_payments: Vec<Payment> =
        pool.prep_exec("SELECT customer_id, amount, account_name FROM tmp.payment", ())
        .map(|result| {
            // In this closure we map QueryResult to Vec<Payment>
            // QueryResult is iterator over MyResult<row, err>
            result.map(|x| x.unwrap()).map(|row| {
                let (customer_id, amount, account_name) = mysql::from_row(row);
                Payment {
                    customer_id: customer_id,
                    amount: amount,
                    account_name: account_name
                }
            }).collect()
        }).unwrap();
    assert_eq!(payments, selected_payments);
    println!("Yay!");
}
