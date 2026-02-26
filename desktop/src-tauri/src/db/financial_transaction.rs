use rusqlite::{Connection, Result};
use crate::db::{FinancialTransaction, get_db_path};

pub fn get_financial_transactions() -> Result<Vec<FinancialTransaction>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, category, description, amount_change, balance, created_at 
         FROM financial_transaction 
         ORDER BY created_at DESC"
    )?;
    let transactions = stmt.query_map([], |row| {
        Ok(FinancialTransaction {
            id: Some(row.get(0)?),
            category: row.get(1)?,
            description: row.get(2)?,
            amount_change: row.get(3)?,
            balance: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    
    transactions.collect()
}

pub fn get_financial_transaction(id: i64) -> Result<Option<FinancialTransaction>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, category, description, amount_change, balance, created_at 
         FROM financial_transaction 
         WHERE id = ?1"
    )?;
    
    let transaction = stmt.query_row([id], |row| {
        Ok(FinancialTransaction {
            id: Some(row.get(0)?),
            category: row.get(1)?,
            description: row.get(2)?,
            amount_change: row.get(3)?,
            balance: row.get(4)?,
            created_at: row.get(5)?,
        })
    });
    
    match transaction {
        Ok(transaction) => Ok(Some(transaction)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn create_financial_transaction(transaction: FinancialTransaction) -> Result<FinancialTransaction> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "INSERT INTO financial_transaction (category, description, amount_change, balance)
         VALUES (?1, ?2, ?3, ?4)",
        [
            &transaction.category as &dyn rusqlite::ToSql,
            &transaction.description as &dyn rusqlite::ToSql,
            &transaction.amount_change as &dyn rusqlite::ToSql,
            &transaction.balance as &dyn rusqlite::ToSql,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    Ok(FinancialTransaction {
        id: Some(id),
        ..transaction
    })
}

pub fn update_financial_transaction(id: i64, transaction: FinancialTransaction) -> Result<FinancialTransaction> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "UPDATE financial_transaction 
         SET category = ?1, description = ?2, amount_change = ?3, balance = ?4 
         WHERE id = ?5",
        [
            &transaction.category as &dyn rusqlite::ToSql,
            &transaction.description as &dyn rusqlite::ToSql,
            &transaction.amount_change as &dyn rusqlite::ToSql,
            &transaction.balance as &dyn rusqlite::ToSql,
            &id as &dyn rusqlite::ToSql,
        ],
    )?;
    
    Ok(FinancialTransaction {
        id: Some(id),
        ..transaction
    })
}

pub fn delete_financial_transaction(id: i64) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "DELETE FROM financial_transaction WHERE id = ?1",
        [id],
    )?;
    
    Ok(())
}

pub fn get_current_balance() -> Result<f64> {
    let conn = Connection::open(get_db_path())?;
    
    let balance = conn.query_row(
        "SELECT balance FROM financial_transaction ORDER BY created_at DESC LIMIT 1",
        [],
        |row| row.get(0)
    );
    
    match balance {
        Ok(b) => Ok(b),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0.0),
        Err(e) => Err(e),
    }
}
