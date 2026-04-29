# causeway-cpi

Anchor CPI wrappers for tenant programs that want to call into the
Causeway threshold-custody program.

Tenant programs that want to issue threshold-signing requests link
against this crate and call `causeway_cpi::request_signing(...)` —
same shape Anchor's `cpi` module offers when you depend directly on a
fellow Anchor program, but without forcing tenants to compile the
whole Causeway program.
