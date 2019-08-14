import numpy as np
import pytest

from .participant import Participant


def test_Participant_x_y_shape_valid():
    # Prepare
    m = None
    x = np.zeros((5, 32, 32, 3), dtype=np.uint8)
    y = np.zeros((5), dtype=np.uint8)
    # Execute
    _ = Participant(0, m, (x, y), (x, y), num_classes=10, batch_size=32)
    # pass if initialization does not raise an exception


def test_Participant_x_y_shape_invalid():
    # Prepare
    m = None
    x = np.zeros((3, 32, 32, 3), dtype=np.uint8)
    y = np.zeros((4), dtype=np.uint8)
    # Execute & assert
    try:
        _ = Participant(0, m, (x, y), (x, y), num_classes=10, batch_size=32)
        pytest.fail("No AssertionError raised")
    except AssertionError:
        pass
