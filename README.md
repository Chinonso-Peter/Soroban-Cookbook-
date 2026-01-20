# Soroban-Cookbook-
**The community-driven smart contract pattern library for Stellar**

## 🌟 Our Mission

We're building the go-to resource for Soroban developers—a comprehensive, battle-tested collection of smart contract examples that make building on Stellar faster, safer, and more accessible.

## 🎯 What We Do

Soroban-Cookbook provides production-ready Rust smart contract patterns for developers at every level:

- **Newcomers** learn blockchain development through annotated, beginner-friendly examples
- **Ethereum migrants** find clear "Solidity → Soroban" translation guides
- **Experienced builders** access optimized templates for complex DeFi and governance systems

## 📚 Our Repositories

### [Soroban-Cookbook](https://github.com/soroban-blueprints/soroban-blueprints)
The main pattern library with atomic snippets, financial primitives, and advanced multi-contract systems.


### [templates](https://github.com/soroban-blueprints/templates)
Ready-to-fork project starters with CI/CD, testing infrastructure, and deployment scripts pre-configured.

## 🚀 Quick Start

```bash
# Clone any example
git clone https://github.com/soroban-blueprints/soroban-blueprints
cd soroban-blueprints/examples/token-wrapper

# Test and deploy
cargo test
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/*.wasm --network testnet
```

### Ways to Contribute

1. **Write Examples** — Create new contract patterns (150-200 pts)
2. **Improve Docs** — Add tutorials, fix typos, translate content (100-150 pts)
3. **Maintain Code** — Update dependencies, add tests, ensure SDK compatibility (100 pts)
4. **Review PRs** — Help maintain quality standards across the library

See [CONTRIBUTING.md](https://github.com/soroban-blueprints/soroban-blueprints/blob/main/CONTRIBUTING.md) for detailed guidelines.

### Current Focus Areas

- ✅ Core primitives (Auth, Storage, Events)
- 🚧 Financial patterns (Escrow, Vaults, AMMs)
- 📋 Governance systems (DAOs, Voting, Proposals)
- 📋 Cross-contract patterns (Composability examples)


## 📊 Impact

Our goal is to become the primary learning resource for Soroban development, measured by:

- GitHub stars and forks from ecosystem developers
- Community adoption in production projects
- Cross-references from official Stellar documentation

## 🛠️ Technical Standards

All patterns in our library:

- ✅ Compile against the latest Soroban SDK
- ✅ Include comprehensive `cargo test` coverage
- ✅ Provide one-command deployment scripts
- ✅ Feature inline documentation explaining design decisions
- ✅ Pass automated CI/CD quality checks

## 🌐 Community

- **Discord**: [Join our server](https://discord.gg/soroban-blueprints)
- **Twitter**: [@SorobanBlue](https://twitter.com/sorobanblue)
- **Forum**: [Stellar Community](https://stellar.org/community)

## 📖 Learn More

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developer Portal](https://developers.stellar.org)

## 📄 License

All repositories are MIT licensed—build freely, attribute generously.

---

**Maintained by the community • Powered by Stellar • Built with Rust**

*Have a pattern idea? [Open an issue](https://github.com/soroban-blueprints/Soroban-Cookbook/issues/new)*
