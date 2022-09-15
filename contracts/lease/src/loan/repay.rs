use finance::{coin::Coin, currency::Currency};

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Receipt<C>
where
    C: Currency,
{
    previous_margin_paid: Coin<C>,
    current_margin_paid: Coin<C>,
    previous_interest_paid: Coin<C>,
    current_interest_paid: Coin<C>,
    principal_paid: Coin<C>,
    close: bool,
}

impl<C> Receipt<C>
where
    C: Currency,
{
    pub fn previous_margin_paid(&self) -> Coin<C> {
        self.previous_margin_paid
    }

    pub fn previous_interest_paid(&self) -> Coin<C> {
        self.previous_interest_paid
    }

    pub fn current_margin_paid(&self) -> Coin<C> {
        self.current_margin_paid
    }

    pub fn current_interest_paid(&self) -> Coin<C> {
        self.current_interest_paid
    }

    pub fn principal_paid(&self) -> Coin<C> {
        self.principal_paid
    }

    pub fn close(&self) -> bool {
        self.close
    }

    pub(crate) fn total(&self) -> Coin<C> {
        self.previous_margin_paid
            + self.previous_interest_paid
            + self.current_margin_paid
            + self.current_interest_paid
            + self.principal_paid
    }

    pub(super) fn pay_previous_margin(&mut self, payment: Coin<C>) {
        debug_assert_eq!(self.previous_margin_paid, Coin::default());

        self.previous_margin_paid = payment;
    }

    pub(super) fn pay_previous_interest(&mut self, payment: Coin<C>) {
        debug_assert_eq!(self.previous_interest_paid, Coin::default());

        self.previous_interest_paid = payment;
    }

    pub(super) fn pay_current_margin(&mut self, payment: Coin<C>) {
        debug_assert_eq!(self.current_margin_paid, Coin::default());

        self.current_margin_paid = payment;
    }

    pub(super) fn pay_current_interest(&mut self, payment: Coin<C>) {
        debug_assert_eq!(self.current_interest_paid, Coin::default());

        self.current_interest_paid = payment;
    }

    pub(super) fn pay_principal(&mut self, principal: Coin<C>, payment: Coin<C>) {
        debug_assert_eq!(self.principal_paid, Coin::default());

        // TODO uncomment when issue #13 is solved
        // debug_assert!(payment <= principal, "Payment exceeds principal!");

        self.principal_paid = payment;

        // TODO change to `==` when issue #13 is solved
        self.close = principal <= payment;
    }
}

#[cfg(test)]
impl<C> Receipt<C>
where
    C: Currency,
{
    pub(crate) fn new(
        previous_margin_paid: Coin<C>,
        current_margin_paid: Coin<C>,
        previous_interest_paid: Coin<C>,
        current_interest_paid: Coin<C>,
        principal_paid: Coin<C>,
        close: bool,
    ) -> Receipt<C> {
        Self {
            previous_margin_paid,
            current_margin_paid,
            previous_interest_paid,
            current_interest_paid,
            principal_paid,
            close,
        }
    }
}

#[cfg(test)]
mod tests {
    use finance::{coin::Coin, currency::Nls};

    use crate::loan::RepayReceipt;

    #[test]
    fn pay_principal_full() {
        let principal = Coin::<Nls>::new(10);

        let mut receipt = RepayReceipt::default();

        receipt.pay_principal(principal, principal);

        assert_eq!(
            receipt,
            RepayReceipt {
                principal_paid: principal,
                close: true,
                ..Default::default()
            },
        );
    }

    #[test]
    // TODO uncomment when issue #13 is solved
    // #[should_panic = "Payment exceeds principal!"]
    fn pay_principal_overpaid() {
        let principal = Coin::<Nls>::new(10);

        let payment = principal + Coin::<Nls>::new(1);

        let mut receipt = RepayReceipt::default();

        receipt.pay_principal(principal, payment);

        assert_eq!(
            receipt,
            RepayReceipt {
                // TODO change to `principal` when issue #13 is solved
                principal_paid: payment,
                close: true,
                ..Default::default()
            },
        );
    }
}
