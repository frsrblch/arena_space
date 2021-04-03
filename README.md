# arena_space

## Resources

- ores are mined and refined into resources
- resources include metals, fuels, foodstuffs, ores
- resources 

## Tech

```
enum Tech { Theory, Application }

impl Tech {
    pub fn get_prereqs(&self) -> &[Self];
}
```

- Theory:
    - Nuclear model
    - Relativity   
    - Quantum mechanics
    - Warp field theory
- Application:
    - Propulsion
        - Rocketry
        - Ion drive
        - Nuclear thermal drive
        - Fusion rocket
        - Impulse drive
        - Warp drive
    - Tunneling
        - Gateway tech
    - Power
        - Fission reactor
        - Fusion reactor
    - Medicine
        - Nuclear medicine
    - Computing
        - Integrated circuits
        - 3D circuits
        - Quantum circuits
    
    