# Copyright 2025 Ubuntu
# See LICENSE file for licensing details.
#
# Learn more about testing at: https://juju.is/docs/sdk/testing

from ops import testing

from charm import QdrantK8SOperatorCharm


def test_pebble_ready():
    # Arrange:
    ctx = testing.Context(QdrantK8SOperatorCharm)
    container = testing.Container("some-container", can_connect=True)
    state_in = testing.State(containers={container})

    # Act:
    state_out = ctx.run(ctx.on.pebble_ready(container), state_in)

    # Assert:
    assert state_out.unit_status == testing.ActiveStatus()
