"""Execute the actual generated Python client against the independent wire oracle."""
import os
from sikaru_api import SikaruApi
from sikaru_api.core.api_error import ApiError


def client(token):
    return SikaruApi(base_url=os.environ['CONTRACT_URL'], api_key=token, max_retries=5)


def create_and_ready(controller, provenance):
    attachment = controller.compute_attachments.create('project', 'session', environment_id='environment',
        idempotency_key='create', workspace_provenance=provenance)
    assert attachment.owner_id is None
    assert attachment.workspace_generation == 'generation' and attachment.status == 'pending'
    assert controller.compute_attachments.get('project', 'attachment').journal_id == 'journal'
    claim = controller.compute_attachments.claim('project', 'attachment', idempotency_key='claim')
    assert claim.attachment_id == 'attachment'
    assert claim.owner_id == 'controller:claim'
    assert claim.owner_epoch == 1
    credential = controller.compute_attachments.issue_credential('project', 'attachment',
        owner_id=claim.owner_id, owner_epoch=claim.owner_epoch)
    assert credential.credential_id == 'executor-credential'
    assert credential.expires_at == 2000000000.0
    executor = client(credential.token)
    ready = executor.compute_attachments.ready('project', 'attachment', executor_instance_id='instance',
        journal_id='journal', workspace_provenance=provenance, protocol_version='sikaru-compute-v1', capabilities=['compute.execute'])
    assert ready.status == 'ready' and ready.lease_ttl_seconds == 60
    return executor


def execute_and_reconcile(executor, provenance):
    work = executor.compute_operations.poll('project', 'attachment', wait_seconds=0)
    assert work.execution_phase == 'running' and work.operations[0].request_digest == 'a'*64
    receipt = dict(run_id='run', tool_call_id='tool', tool_provider_id='provider', capability_name='compute.execute',
        idempotency_key='receipt', request_digest='a'*64, status='completed', payload={'output':'done'})
    assert executor.compute_operations.submit_receipt('project', 'attachment', **receipt).created
    assert not executor.compute_operations.submit_receipt('project', 'attachment', **receipt).created
    reconciled = executor.compute_attachments.reconcile('project', 'attachment', executor_instance_id='instance',
        journal_id='journal', workspace_provenance=provenance,
        processes=[{'handle_id':'handle','status':'lost','evidence':'ownership unproven'}])
    assert reconciled.attachment.status == 'recovery_required'
    assert executor.compute_attachments.status('project', 'attachment').processes[0].status == 'lost'
    terminal = executor.compute_operations.poll('project', 'attachment')
    assert terminal.execution_phase == 'terminal' and terminal.execution.terminal
    return receipt


def verify_failures(executor, receipt):
    try:
        client('wrong-scope').compute_operations.poll('project', 'attachment')
        raise AssertionError('unauthorized request succeeded')
    except ApiError as error:
        assert error.status_code == 403
    try:
        executor.compute_operations.submit_receipt('project', 'attachment', **receipt, request_options={'max_retries':9})
        raise AssertionError('unavailable mutation succeeded')
    except ApiError as error:
        assert error.status_code == 503


def main():
    controller = client('controller')
    provenance = {'kind': 'existing_directory', 'identity': 'workspace'}
    executor = create_and_ready(controller, provenance)
    receipt = execute_and_reconcile(executor, provenance)
    verify_failures(executor, receipt)


if __name__ == '__main__':
    main()
