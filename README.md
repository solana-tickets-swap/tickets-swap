# Tickets Swap

## Groupe 23:
| Nom | GitHub |
|:----:|:---:|
| Stephen Damian | [@s-damian](https://github.com/s-damian) |
| Thomas Dupuy | [@tomaka7](https://github.com/tomaka7) |
| Cécile Hirschauer | [@Cecile-Hirschauer](https://github.com/Cecile-Hirschauer) |

lien du site: [Tickets-Swap](https://www.tickets-swap.daw-dev.fr/)

## Transactions

Pour consulter les transactions de notre programme, veuillez visiter le lien suivant : [Explorer Solana - Transactions du programme](https://explorer.solana.com/address/E3Mqfc5uYhQ1V8VCNQpWnx59LXECGYo3fvfvFgxFq1Ah?cluster=devnet)

Exemple d'un NFT créé : [Explorer Solana - Exemple d'un NFT](https://explorer.solana.com/address/NNZTRskfa387bCwZv28MyHwqmJt9qRqRxK2cskfBzTf?cluster=devnet)


## Présentation

Une plateforme pour vendre des billets d'événements (concerts, spectacles, conférences) sous forme de NFT, permettant une vérification facile et sécurisée des billets.

## Technologies

- **Anchor** : Framework pour le développement de smart contracts sur Solana.
- **Next.js** : Framework React pour le développement de sites web et d'applications.
- **Solana-Web3.js** : Bibliothèque JavaScript pour interagir avec la blockchain Solana.
- **metaplex_metadata_token** : Programme pour la gestion des métadonnées des tokens NFT sur la blockchain Solana.
- **spl-token** : Librairie pour la gestion des tokens sur Solana.
- **Tailwind CSS** : Framework CSS pour la conception de styles rapides et modulaires.
- **Phantom Wallet** : Wallet Solana utilisé pour interagir avec les applications décentralisées.

## Structure du projet

```plaintext
app/
  tickets-swap // Front-End in Next.js
    app/
        // Les pages.
    src/
        components/
        handlers/
        idl/
        utils/
migrations/
node_modules/
programs/
  tickets-swap/
    src/
      kernel/
        event_manager.rs
        mod.rs
        nft_manager.rs
        ticket_manager.rs
      lib.rs
  Cargo.toml
  Xargo.toml
sh/
  copy-idl.sh
  test-ledger.sh
target/
tests/
  metaplex_token_metadata_program.so
  tests.ts
.gitignore
.prettierignore
.prettierrc.json
Anchor.toml
Cargo.lock
Cargo.toml
package-lock.json
package.json
README.md
tsconfig.json
```

## Fonctionnalités Principales
- Gestion des Événements
    - **Création d'Événements** :
        - Permet aux utilisateurs de créer des événements avec un titre, une description, une date, un lieu et un prix de billet.
- Gestion des Tickets
    - **Achat de Tickets :**
        - Permet aux utilisateurs d'acheter des tickets pour des événements.
        - Assigne le ticket à l'acheteur et transfère le montant du ticket à l'organisateur de l'événement.
        - Vérification des tickets.
- Gestion des NFTs
    - **Création et mint de NFTs pour les Tickets :**
        - Permet de créer un NFT associé à un ticket, avec un nom, un symbole et une URI.
        - Vérifie l'autorisation du signataire et si le ticket a déjà un NFT.

## Tests
Pour exécuter les tests, utilisez la commande suivante :

```bash
anchor test --skip-local-validator
```

Résultats des Tests
Création d'un événement et d'un ticket :
```
createEvent - Transaction réussie.
buyTicket - Tentative d'achat de billet avec un propriétaire invalide.
buyTicket - Tentative d'achat de billet réussie.
createNft - Tentative de création d'un NFT réussie.
verifyNft - Vérifier ticket appartient à l'événement.

5 passing (3.67s)
```
## Déploiement
Le programme a été déployé en local et sur un custom RPC Devnet QuickNode :

```

Cluster: <RPC_URL>
Upgrade authority: ~/.config/solana/id.json
Program Id: <YOUR_PROGRAM_ID>
```
