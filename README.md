# ase_extension

Extension to ASE(atomistic simulation environment), written in Rust and Python.

## Features

- `ase_extension.constraints.LogFermiWallPotential`: Constrain atoms in sphere with logfermi potential
- `ase_extension.geometry.RMSD`: Minimum RMSD between molecules and its gradient

Note that these features does not support PBC properly.

## Installation


### With pypi

pip install ase_extension

### From source 

Install `maturin` first, and do:
```bash
pip install git+https://github.com/mjhong0708/ase_extension
```

## User guide

See [wiki](https://github.com/mjhong0708/ase_extension/wiki).


## Features to add

- [ ] Neighbor list (using [gchemol-neighbors](https://crates.io/crates/gchemol-neighbors))
- [ ] Molecule packer for MD (like packmol, but supports PBC)
