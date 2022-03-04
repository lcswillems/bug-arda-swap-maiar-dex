#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait ArdaSwap {
  #[init]
  fn init(&self) {}

  #[payable("*")]
  #[endpoint(acceptPay)]
  fn accept_pay(&self) {}

  #[payable("*")]
  #[endpoint(swap)]
  fn swap(
    &self,
    #[payment_token] token_in_id: TokenIdentifier,
    #[payment_amount] amount_in: BigUint,
    pair_address: ManagedAddress,
    token_out_id: TokenIdentifier,
  ) {
    self.maiar_pair_proxy(pair_address)
      .swap_tokens_fixed_input(
        token_in_id, 0, amount_in, token_out_id, BigUint::zero(),
        OptionalValue::Some(ManagedBuffer::from(ACCEPT_PAY_FUNC_NAME))
      )
      .execute_on_dest_context_custom_range(|_, after| (after - 1, after));
  }

  #[proxy]
  fn maiar_pair_proxy(&self, to: ManagedAddress) -> maiar_pair::Proxy<Self::Api>;
}

const ACCEPT_PAY_FUNC_NAME: &[u8] = b"acceptPay";

pub mod maiar_pair {
  elrond_wasm::imports!();
  elrond_wasm::derive_imports!();

  #[elrond_wasm::proxy]
  pub trait Pair {
    #[payable("*")]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
      &self,
      #[payment_token] token_in: TokenIdentifier,
      #[payment_nonce] nonce: u64,
      #[payment_amount] amount_in: BigUint,
      token_out: TokenIdentifier,
      amount_out_min: BigUint,
      #[var_args] opt_accept_funds_func: OptionalValue<ManagedBuffer>,
    ) -> EsdtTokenPayment<Self::Api>;
  }
}
