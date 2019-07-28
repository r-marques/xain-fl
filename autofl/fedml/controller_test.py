from .controller import CycleRandomController, RoundRobinController


def test_round_robin_controller():
    # Prepare
    NUM_PARTICIPANTS = 3
    controller = RoundRobinController(num_participants=NUM_PARTICIPANTS)
    # Execute & assert
    for _ in range(5):
        for i in range(NUM_PARTICIPANTS):
            # Execute
            indices = controller.indices(num_indices=1)
            # Assert
            assert len(indices) == 1
            assert indices[0] == i


def test_cycle_random_controller():
    # Prepare
    NUM_PARTICIPANTS = 3
    controller = CycleRandomController(num_participants=NUM_PARTICIPANTS)

    # Execute
    indices0 = []
    for _ in range(NUM_PARTICIPANTS):
        indices0.append(controller.indices(num_indices=1)[0])
    indices1 = []
    for _ in range(NUM_PARTICIPANTS):
        indices1.append(controller.indices(num_indices=1)[0])

    # Assert
    assert len(indices0) == 3
    assert set(indices0) == set([0, 1, 2])
    assert len(indices1) == 3
    assert set(indices1) == set([0, 1, 2])
