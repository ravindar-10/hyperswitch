use diesel::{associations::HasTable, BoolExpressionMethods, ExpressionMethods, Table};

use super::generics;
use crate::{
    enums as storage_enums, errors,
    payment_method::{self, PaymentMethod, PaymentMethodNew},
    schema::payment_methods::dsl,
    PgPooledConn, StorageResult,
};

impl PaymentMethodNew {
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<PaymentMethod> {
        generics::generic_insert(conn, self).await
    }
}

impl PaymentMethod {
    pub async fn delete_by_payment_method_id(
        conn: &PgPooledConn,
        payment_method_id: String,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            dsl::payment_method_id.eq(payment_method_id),
        )
        .await
    }

    pub async fn delete_by_merchant_id_payment_method_id(
        conn: &PgPooledConn,
        merchant_id: &str,
        payment_method_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_method_id.eq(payment_method_id.to_owned())),
        )
        .await
    }

    pub async fn find_by_locker_id(conn: &PgPooledConn, locker_id: &str) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::locker_id.eq(locker_id.to_owned()),
        )
        .await
    }

    pub async fn find_by_payment_method_id(
        conn: &PgPooledConn,
        payment_method_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::payment_method_id.eq(payment_method_id.to_owned()),
        )
        .await
    }

    pub async fn find_by_merchant_id(
        conn: &PgPooledConn,
        merchant_id: &str,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<
            <Self as HasTable>::Table,
            _,
            <<Self as HasTable>::Table as Table>::PrimaryKey,
            _,
        >(
            conn,
            dsl::merchant_id.eq(merchant_id.to_owned()),
            None,
            None,
            None,
        )
        .await
    }

    pub async fn find_by_customer_id_merchant_id(
        conn: &PgPooledConn,
        customer_id: &str,
        merchant_id: &str,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id
                .eq(customer_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned())),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn find_by_customer_id_merchant_id_status(
        conn: &PgPooledConn,
        customer_id: &str,
        merchant_id: &str,
        status: storage_enums::PaymentMethodStatus,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id
                .eq(customer_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                .and(dsl::status.eq(status)),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn update_with_payment_method_id(
        self,
        conn: &PgPooledConn,
        payment_method: payment_method::PaymentMethodUpdate,
    ) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(
            conn,
            dsl::payment_method_id.eq(self.payment_method_id.to_owned()),
            payment_method::PaymentMethodUpdateInternal::from(payment_method),
        )
        .await
        {
            Err(error) => match error.current_context() {
                errors::DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }
}
