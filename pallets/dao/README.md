# Pallet DAO
This pallet is used to manage a simple on-chain DAO, with a configurable and immutable maximum number of weights.
Accounts can join the DAO, by paying a certain membership fee, designed to be upgradable. Members can leave the DAO as well.

## Random Generation
Members of the DAO can participate to a collective randomness generation process, and stake some of their funds, to have the chance to earn a reward.
A member who commits to a number, but later does not reveal it, is slashed by 25%.
A member can freely change thir commitment, until the reveal phase starts.

### Random Generation Process
The random generation process is composed of 3 phases:
- Commit phase (10 blocks): members can commit to a number, and stake some funds;
- Reveal phase (10 blocks): members reveal their number, which are used to generate a random number by XORing them;
- Cooldown phase (80 blocks): a new random number has been generated.

It is designed to be repeated every 100 blocks, usually 10 minutes.

## Problems and Missing Features
- Currently, there is no way to pay the DAO to read the random generated number. However, one could read the value by calling the function `Dao::current_random()`.
- Consequently, there is no way for a member to withdraw the staking reward, if any.
- During the reveal phase, the random number is 0 until the first reveal is made. This is not a problem, since the random number is XORed with the commitment, but it is not ideal.
- There's no way to spend the DAO's money, or to upgrade the membership fee.
