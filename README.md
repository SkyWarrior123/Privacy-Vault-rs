# Data Privacy Vault (Rust)

A small HTTP service that acts as a Data Privacy Vault for the CodingChallenges.fyi “Build Your Own Data Privacy Vault” challenge.

The idea is simple:

- Send sensitive data to the vault over HTTP.
- Get back tokens.
- Later, send tokens to the vault to recover the original data (if you are allowed to).