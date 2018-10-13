use types::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ledger {
	// TODO: can the strings be Rc<Hash>?
	latest_tx_entry_hash: Hash, // TODO: is this necessary?
	seq_no: u64, // TODO: is this necessary?
	min_timeout: u32, // TODO: could these be in seq_no units?
	max_timeout: u32,
	max_pending_htls: u8,
	max_ops_per_transaction: u8,

    id: Hash,
    issuer: Hash,
    owner: Hash,
	limit: u128,
    balance: u128,
    exchange_rate_n: u64,
    exchange_rate_d: u64,
	metadata: String,
}

// TODO: needs logic to:
/*  dictate how basic transactions and operations are applied and can be synchronized/caught up
 */
/*  dictate how a (basic or HTL) transaction can be applied to a ledger with pending HTL(s)
	- updates to general fields:
		- updates made by one party with static invariants
		- e.g. exchange_rate
	- updates to limit:
		- updates made by one party with static (e.g. increase must be less than max_u128) and field (e.g. decrease can't be below balance) invariants
	- updates to seq_no:
		- updates made by either party with static invariants (must be one greater than seq_no on current ledger)
	- updates to balance:
		- updates made by either party with static and field invariants
 */
/*	dictate if and how an HTL (nested behind other applied transactions) is failed/fulfilled
	- any new failed/fulfilled htl tx:
		- update original tx as marked as failed/fulfilled
		- if this fails/fulfills the first pending htl in the list (i.e. the oldest):
			- remove it from the list, apply it to ledger(s)
			- for every subsequent txn, until the next pending htl:
				- if basic: apply it to ledger(s)
				- if htl and failed/fulfilled: remove from list, apply it to ledger
			- at this point, the decided history of confirmed txns should be already applied to the ledger(s), so commit it/them
		- else (if it fails/fulfills an htl in the middle of the list):
			- remove it from list
 */
impl Ledger {
    pub fn new(
        id: Hash,
		owner: Hash,
        issuer: Hash,
		latest_tx_entry_hash: Hash,
		min_timeout: u32,
		max_timeout: u32,
		max_pending_htls: u8,
		max_ops_per_transaction: u8,
        limit: u128,
        balance: u128,
        exchange_rate_n: u64,
        exchange_rate_d: u64,
		metadata: String,
    ) -> Ledger {
        Ledger {
            id,
            owner,
            issuer,
            seq_no: 0,
			min_timeout,
			max_timeout,
			max_pending_htls,
			max_ops_per_transaction,
            limit,
            balance,
            exchange_rate_n,
            exchange_rate_d,
			metadata,
			latest_tx_entry_hash,
        }
    }

	// GETTERS
	pub fn id(&self) -> &Hash { &self.id }
	pub fn owner(&self) -> &Hash { &self.owner }
	pub fn issuer(&self) -> &Hash { &self.issuer }

	pub fn seq_no(&self) -> u64 { self.seq_no }
	pub fn limit(&self) -> u128 { self.limit }
	pub fn balance(&self) -> u128 { self.balance }
	pub fn exchange_rate(&self) -> (u64, u64) {
		(self.exchange_rate_n, self.exchange_rate_d)
	}
	pub fn metadata(&self) -> &str { &self.metadata }

	// SETTERS
	pub fn bump_seq_no(&mut self, new_seq_no: Option<u64>) -> u64 {
		match new_seq_no {
			None => self.seq_no += 1,
			Some(seq_no) => self.seq_no = seq_no,
		}
		self.seq_no
	}
	pub fn set_limit(&mut self, limit: u128) -> &mut Self {
		self.limit = limit;
		self
	}
	pub fn set_balance(&mut self, balance: u128) -> &mut Self {
		self.balance = balance;
		self
	}
	pub fn set_exchange_rate(&mut self, rate: (u64, u64)) -> &mut Self {
		self.exchange_rate_n = rate.0;
		self.exchange_rate_d = rate.1;
		self
	}

	// pub fn set_metadata(&self) -> &str { &self.metadata }

    // fn from_json(json: serde_json::Value) -> Result<Ledger, serde_json::Error> {
    //     serde_json::from_value(json)
    // }

    // fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
    //     serde_json::to_value(self)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ledger = Ledger::new(
			String::from("randomID"),
			String::from("alice"),
			String::from("bob"),
			String::from(""),
			30000,
			60000,
			5,
			5,
			u128::max_value(),
			u128::max_value(),
			1,
			1,
			String::from("")
		);

        println!("res: {:#?}", &ledger);

		assert!(true);
    }
}

