from aseext import aseext as _ext
import numpy as np


def neighbor_list(
    positions: np.ndarray,
    cell: np.ndarray,
    pbc: list[bool],
    cutoff: float,
    self_interaction: bool,
):
    return _ext.neighbor_list(positions, cell, pbc, cutoff, self_interaction)


def distance_matrix(
    x: np.ndarray,
    y: np.ndarray,
    par_threshold_x: int = 10,
    par_threshold_y: int = 10,
):
    par_x = False
    par_y = False

    if x.shape[0] > par_threshold_x:
        par_x = True
    if y.shape[0] > par_threshold_y:
        par_y = True
    # print(f"par_x: {par_x}, par_y: {par_y}")
    return _ext.distance_matrix(x, y, par_x, par_y)
