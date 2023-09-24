# Pallet DAO
This pallet is used to manage a simple on-chain DAO, with a configurable and immutable maximum number of weights.  
Accounts can join the DAO, by paying a certain membership fee, designed to be upgradable. Members can leave the DAO as well.

## Random Generation
Members of the DAO can participate to a collective randomness generation process, and stake some of their funds, to have the chance to earn a reward.  
The reward is distributed to the members of the DAO, proportionally to their stake.  
A user who commits to a number, but later does not reveal it, is slashed by 25%.

### Random Generation Process
The random generation process is composed of 3 phases:
- Commit phase (10 blocks): members can commit to a number, and stake some funds;
- Reveal phase (10 blocks): members reveal their number;
- Cooldown phase (80 blocks): a new random number has been generated.

It is designed to be repeated every 100 blocks, usually 10 minutes.
