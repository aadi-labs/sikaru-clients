# Reference
## Auth
<details><summary><code>client.auth.<a href="src/sikaru_api/auth/client.py">get_device_configuration</a>() -> DeviceConfiguration</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.auth.get_device_configuration()

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## HttpChannels
<details><summary><code>client.http_channels.<a href="src/sikaru_api/http_channels/client.py">invoke</a>(...) -> HttpReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.http_channels.invoke(
    binding_id="binding_id",
    content="content",
    conversation_id="conversation_id",
    message_id="message_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `str` — Text content, at most 16 KiB encoded as UTF-8.
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**actor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**privacy:** `typing.Optional[HttpMessagePrivacy]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.http_channels.<a href="src/sikaru_api/http_channels/client.py">poll</a>(...) -> HttpReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.http_channels.poll(
    binding_id="binding_id",
    receipt_id="receipt_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.http_channels.<a href="src/sikaru_api/http_channels/client.py">start_slack_link</a>(...) -> PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.http_channels.start_slack_link(
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `SlackLink` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.http_channels.<a href="src/sikaru_api/http_channels/client.py">unlink_slack_identity</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.http_channels.unlink_slack_identity(
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Completions
<details><summary><code>client.completions.<a href="src/sikaru_api/completions/client.py">create_stream</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.completions.create_stream(
    model="model",
    prompt="prompt",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `CreateCompletionsStreamRequestPrompt` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**best_of:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**echo:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**logit_bias:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**min_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**n:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**repetition_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**seed:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**stop:** `typing.Optional[CreateCompletionsStreamRequestStop]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**suffix:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**top_k:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**typical_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.completions.<a href="src/sikaru_api/completions/client.py">create</a>(...) -> Completion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.completions.create_stream(
    model="model",
    prompt="prompt",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `CreateCompletionsRequestPrompt` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**best_of:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**echo:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**logit_bias:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**min_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**n:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**repetition_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**seed:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**stop:** `typing.Optional[CreateCompletionsRequestStop]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**suffix:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**top_k:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**typical_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Models
<details><summary><code>client.models.<a href="src/sikaru_api/models/client.py">list</a>() -> ModelList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.models.list()

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.models.<a href="src/sikaru_api/models/client.py">retrieve</a>(...) -> PublicModel</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.models.retrieve(
    model="model",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PersonalChannels
<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">get_message</a>(...) -> PersonalChannelReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.get_message(
    binding_id="binding_id",
    receipt_id="receipt_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">authorize_connection</a>(...) -> ConnectionAuthorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.authorize_connection(
    binding_id="binding_id",
    receipt_id="receipt_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">complete_connection</a>(...) -> PersonalChannelConnection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.complete_connection(
    binding_id="binding_id",
    receipt_id="receipt_id",
    connection_id="connection_id",
    state="state",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CompleteAuthorization` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">replace_connection_credentials</a>(...) -> PersonalChannelConnection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ConnectionCredentials
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.replace_connection_credentials(
    binding_id="binding_id",
    receipt_id="receipt_id",
    connection_id="connection_id",
    credentials=ConnectionCredentials(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ReplaceCredentials` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">list_files</a>(...) -> PersonalChannelFiles</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.list_files(
    binding_id="binding_id",
    receipt_id="receipt_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">download_file</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.download_file(
    binding_id="binding_id",
    receipt_id="receipt_id",
    file_id="file_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">decide_approval</a>(...) -> PersonalChannelApproval</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.decide_approval(
    binding_id="binding_id",
    receipt_id="receipt_id",
    tool_call_id="tool_call_id",
    decision="approved",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tool_call_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ApprovalInput` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">get_slack_link</a>(...) -> PersonalSlackStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.get_slack_link(
    binding_id="binding_id",
    verification_id="verification_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">start_slack_link</a>(...) -> PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.start_slack_link(
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `SlackLink` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personal_channels.<a href="src/sikaru_api/personal_channels/client.py">unlink_slack_identity</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.personal_channels.unlink_slack_identity(
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_user_assertion:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_member_authorization:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Activation
<details><summary><code>client.activation.<a href="src/sikaru_api/activation/client.py">project_activation_status</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.activation.project_activation_status(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AgentDocuments
<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">draft</a>(...) -> AgentDocumentDraft</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Draft a first agent document, starter checks and suggested apps from a description or past conversations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.draft(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**note:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**run_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">import_files</a>(...) -> ImportedDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.import_files(
    project_id="project_id",
    files={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**files:** `typing.Dict[str, str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">list_resources</a>(...) -> DocumentResources</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.list_resources(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">edit_setting</a>(...) -> ImportedDocument</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Set or remove one setting without rewriting the rest of the document.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.edit_setting(
    project_id="project_id",
    document="document",
    path=[
        "path"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**document:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**path:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**op:** `typing.Optional[EditDocumentSettingOp]` 
    
</dd>
</dl>

<dl>
<dd>

**value:** `typing.Optional[typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">list_templates</a>(...) -> DocumentTemplates</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.list_templates(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">validate_text</a>(...) -> DocumentValidationView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.validate_text(
    project_id="project_id",
    document="document",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `DocumentInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">get</a>(...) -> AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.get(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">save</a>(...) -> AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.save(
    project_id="project_id",
    agent_slug="agent_slug",
    document="document",
    expected_revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**document:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**expected_revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">compare</a>(...) -> DocumentComparison</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.compare(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">discard</a>(...) -> AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.discard(
    project_id="project_id",
    agent_slug="agent_slug",
    expected_revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**expected_revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**expected_live_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">publish</a>(...) -> DocumentPublication</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.publish(
    project_id="project_id",
    agent_slug="agent_slug",
    revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_removals:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_widening:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_access_digest:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_live_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">revert</a>(...) -> DocumentPublication</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.revert(
    project_id="project_id",
    agent_slug="agent_slug",
    harness_version_id="harnessVersionId",
    revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_version_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_removals:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_widening:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_access_digest:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_live_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">review</a>(...) -> DocumentReview</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.review(
    project_id="project_id",
    agent_slug="agent_slug",
    revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_removals:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_widening:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_access_digest:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**expected_live_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**harness_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">snippets</a>(...) -> AgentSnippets</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.snippets(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">list_suggestions</a>(...) -> DocumentSuggestions</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.list_suggestions(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">adopt_suggestion</a>(...) -> AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.adopt_suggestion(
    project_id="project_id",
    agent_slug="agent_slug",
    suggestion_id="suggestion_id",
    expected_revision=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**suggestion_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**expected_revision:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">dismiss_suggestion</a>(...) -> DocumentSuggestion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.dismiss_suggestion(
    project_id="project_id",
    agent_slug="agent_slug",
    suggestion_id="suggestion_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**suggestion_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">validate</a>(...) -> DocumentValidationView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.validate(
    project_id="project_id",
    agent_slug="agent_slug",
    document="document",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `DocumentInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_documents.<a href="src/sikaru_api/agent_documents/client.py">list_versions</a>(...) -> DocumentVersions</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_documents.list_versions(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AgentImports
<details><summary><code>client.agent_imports.<a href="src/sikaru_api/agent_imports/client.py">list_agent_imports</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_imports.list_agent_imports(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent_imports.<a href="src/sikaru_api/agent_imports/client.py">create_agent_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, AgentImportImproveRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agent_imports.create_agent_import(
    project_id="project_id",
    improve=AgentImportImproveRequest(
        mode="mode",
        objective="objective",
        promotion_gate="promotionGate",
    ),
    name="name",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**improve:** `AgentImportImproveRequest` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**eval_suites:** `typing.Optional[typing.List[AgentImportEvalSuiteRequest]]` 
    
</dd>
</dl>

<dl>
<dd>

**model_capture:** `typing.Optional[AgentImportModelCaptureRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**runner:** `typing.Optional[AgentImportRunnerRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**source_refs:** `typing.Optional[typing.List[AgentImportSourceRefRequest]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents
<details><summary><code>client.agents.<a href="src/sikaru_api/agents/client.py">create_managed_session</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.agents.create_managed_session(
    project_id="project_id",
    agent_id="agent_id",
    environment_id="environmentId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**external_run_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**external_thread_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CapabilityCeilings
<details><summary><code>client.capability_ceilings.<a href="src/sikaru_api/capability_ceilings/client.py">get</a>(...) -> CapabilityCeilingsView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.capability_ceilings.get(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capability_ceilings.<a href="src/sikaru_api/capability_ceilings/client.py">update</a>(...) -> CapabilityCeilingsView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.capability_ceilings.update(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CapabilityCeilings` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Changesets
<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">list_changesets</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.list_changesets(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[ListChangesetsChangesetsRequestStatus]` 
    
</dd>
</dl>

<dl>
<dd>

**improvement:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">create_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.create_changeset(
    project_id="project_id",
    public_summary="publicSummary",
    public_title="publicTitle",
    scope_ref="scopeRef",
    scope_type="global",
    source_type="import",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**public_summary:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**public_title:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**scope_ref:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**scope_type:** `ScopeType` 
    
</dd>
</dl>

<dl>
<dd>

**source_type:** `SourceType` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**base_release_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**candidate_release_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">get_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.get_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">approve_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ReleaseActionRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.approve_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
    request=ReleaseActionRequest(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ReleaseActionRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">list_changeset_diffs</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.list_changeset_diffs(
    project_id="project_id",
    changeset_id="changeset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">list_changeset_evidence</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.list_changeset_evidence(
    project_id="project_id",
    changeset_id="changeset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">promote_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ReleaseActionRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.promote_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
    request=ReleaseActionRequest(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ReleaseActionRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">reject_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ReleaseActionRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.reject_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
    request=ReleaseActionRequest(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ReleaseActionRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">rollback_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ReleaseActionRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.rollback_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
    request=ReleaseActionRequest(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ReleaseActionRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="src/sikaru_api/changesets/client.py">stage_changeset</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ReleaseActionRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.changesets.stage_changeset(
    project_id="project_id",
    changeset_id="changeset_id",
    request=ReleaseActionRequest(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ReleaseActionRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Channels
<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">availability</a>(...) -> Availability</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.availability(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">bindings</a>(...) -> typing.List[Binding]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.bindings(
    project_id="project_id",
    agent_id="agent_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">create_binding</a>(...) -> Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.create_binding(
    project_id="project_id",
    agent_id="agent_id",
    transport="http",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**transport:** `CreateBindingTransport` 
    
</dd>
</dl>

<dl>
<dd>

**destination_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**destination_kind:** `typing.Optional[CreateBindingDestinationKind]` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**recipient_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">delete_http_binding</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.delete_http_binding(
    project_id="project_id",
    binding_id="binding_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">status</a>(...) -> Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.status(
    project_id="project_id",
    binding_id="binding_id",
    status="active",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `BindingStatusStatus` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">configure_http_binding</a>(...) -> Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.configure_http_binding(
    project_id="project_id",
    binding_id="binding_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**pending_cap:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**per_minute:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">issue_http_credential</a>(...) -> HttpCredential</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.issue_http_credential(
    project_id="project_id",
    binding_id="binding_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">configure_personal_access</a>(...) -> PersonalAccessConfiguration</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.configure_personal_access(
    project_id="project_id",
    binding_id="binding_id",
    enabled=True,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**identity_app_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">recent_receipts</a>(...) -> typing.List[ReceiptSummary]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.recent_receipts(
    project_id="project_id",
    binding_id="binding_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">get_member_slack_link</a>(...) -> PersonalSlackStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.get_member_slack_link(
    project_id="project_id",
    binding_id="binding_id",
    verification_id="verification_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">start_member_slack_link</a>(...) -> PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.start_member_slack_link(
    project_id="project_id",
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `SlackLink` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">unlink_member_slack_identity</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.unlink_member_slack_identity(
    project_id="project_id",
    binding_id="binding_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">save_creation_resume</a>(...) -> CreationResumeId</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.save_creation_resume(
    project_id="project_id",
    payload={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CreationResume` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">get_creation_resume</a>(...) -> CreationResume</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.get_creation_resume(
    project_id="project_id",
    resume_id="resume_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**resume_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">list_identity_apps</a>(...) -> typing.List[ChannelIdentityApp]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.list_identity_apps(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">create_identity_app</a>(...) -> ChannelIdentityAppId</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.create_identity_app(
    project_id="project_id",
    audience="audience",
    issuer="issuer",
    public_jwk={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**audience:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**issuer:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**public_jwk:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**connection_callback_url:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**private_channel_url:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">revoke_identity_app</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.revoke_identity_app(
    project_id="project_id",
    app_id="app_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">update_identity_app_urls</a>(...) -> ChannelIdentityAppUrLs</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.update_identity_app_urls(
    project_id="project_id",
    app_id="app_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_callback_url:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**private_channel_url:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">revoke_subject</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.revoke_subject(
    project_id="project_id",
    app_id="app_id",
    subject="subject",
    tenant_id="tenant_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">create_personal_slack_binding</a>(...) -> Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.create_personal_slack_binding(
    project_id="project_id",
    agent_id="agent_id",
    identity_app_id="identity_app_id",
    installation_id="installation_id",
    verification_id="verification_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**identity_app_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">delivery</a>(...) -> typing.Optional[Delivery]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.delivery(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">resend_delivery</a>(...) -> Delivery</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.resend_delivery(
    project_id="project_id",
    run_id="run_id",
    acknowledge_possible_duplicate=True,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_possible_duplicate:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">dm_status</a>(...) -> DmStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.dm_status(
    project_id="project_id",
    verification_id="verification_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">installations</a>(...) -> typing.List[Installation]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.installations(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">disconnect</a>(...) -> Installation</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.disconnect(
    project_id="project_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">start_dm</a>(...) -> DmChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.start_dm(
    project_id="project_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">rooms</a>(...) -> typing.List[Room]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.rooms(
    project_id="project_id",
    installation_id="installation_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">complete</a>(...) -> OAuthResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.complete(
    project_id="project_id",
    state="state",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**state:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**error:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="src/sikaru_api/channels/client.py">start</a>(...) -> Authorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.channels.start(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**resume_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeAttachments
<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">get</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.get(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">abandon</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Irreversibly abandon execution authority, preserving unknown effects. Cleanup is still required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.abandon(
    project_id="project_id",
    attachment_id="attachment_id",
    evidence="evidence",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**evidence:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">cancel</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Idempotently request stopping. This never asserts child cleanup.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.cancel(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">claim</a>(...) -> ClaimView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Same attachment/key returns the same launch identity, never another launch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.claim(
    project_id="project_id",
    attachment_id="attachment_id",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">cleanup</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.cleanup(
    project_id="project_id",
    attachment_id="attachment_id",
    children_terminated=True,
    evidence="evidence",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**children_terminated:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**evidence:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">connect</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Attest original workspace/journal. Reconnection never replays uncertain effects.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, WorkspaceProvenance
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.connect(
    project_id="project_id",
    attachment_id="attachment_id",
    capabilities=[
        "compute.execute"
    ],
    executor_instance_id="executor_instance_id",
    journal_id="journal_id",
    protocol_version="sikaru-compute-v1",
    workspace_provenance=WorkspaceProvenance(
        identity="identity",
        kind="existing_directory",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ReadyInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">issue_credential</a>(...) -> CredentialIssued</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.issue_credential(
    project_id="project_id",
    attachment_id="attachment_id",
    owner_epoch=1,
    owner_id="owner_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**owner_epoch:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**owner_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">heartbeat</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.heartbeat(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">ready</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, WorkspaceProvenance
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.ready(
    project_id="project_id",
    attachment_id="attachment_id",
    capabilities=[
        "compute.execute"
    ],
    executor_instance_id="executor_instance_id",
    journal_id="journal_id",
    protocol_version="sikaru-compute-v1",
    workspace_provenance=WorkspaceProvenance(
        identity="identity",
        kind="existing_directory",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ReadyInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">reconcile</a>(...) -> ReconcileView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, WorkspaceProvenance
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.reconcile(
    project_id="project_id",
    attachment_id="attachment_id",
    executor_instance_id="executor_instance_id",
    journal_id="journal_id",
    workspace_provenance=WorkspaceProvenance(
        identity="identity",
        kind="existing_directory",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**executor_instance_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**journal_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workspace_provenance:** `WorkspaceProvenance` 
    
</dd>
</dl>

<dl>
<dd>

**processes:** `typing.Optional[typing.List[ProcessObservation]]` 
    
</dd>
</dl>

<dl>
<dd>

**receipts:** `typing.Optional[typing.List[ReceiptInput]]` 
    
</dd>
</dl>

<dl>
<dd>

**uncertain_operation_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">status</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.status(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">stop</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Executor can stop its own attachment and cancel its runs; cleanup is separately reported.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.stop(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">teardown</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Controller attests sandbox teardown; clean parked turns stay resumable.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.teardown(
    project_id="project_id",
    attachment_id="attachment_id",
    children_terminated=True,
    evidence="evidence",
    owner_epoch=1,
    owner_id="owner_id",
    workspace_generation="workspace_generation",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**children_terminated:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**evidence:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**owner_epoch:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**owner_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workspace_generation:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_attachments.<a href="src/sikaru_api/compute_attachments/client.py">create</a>(...) -> AttachmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Same session/key/workspace returns the original binding; changed workspace conflicts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, WorkspaceProvenance
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_attachments.create(
    project_id="project_id",
    session_id="session_id",
    environment_id="environment_id",
    idempotency_key="idempotency_key",
    workspace_provenance=WorkspaceProvenance(
        identity="identity",
        kind="existing_directory",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workspace_provenance:** `WorkspaceProvenance` 
    
</dd>
</dl>

<dl>
<dd>

**replace_existing:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeOperations
<details><summary><code>client.compute_operations.<a href="src/sikaru_api/compute_operations/client.py">submit_receipt</a>(...) -> ReceiptView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Only exact immutable receipt retries are idempotent; changed content conflicts.

A 409 is final for this receipt: it can never be recorded as sent (for example a
receipt a waiting harness already took). Executors settle delivery and keep the
receipt to answer a later issue of the same operation; they never retry a 409.
Network failures, 429 and 5xx are transient and may be retried.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_operations.submit_receipt(
    project_id="project_id",
    attachment_id="attachment_id",
    idempotency_key="idempotency_key",
    payload={
        "key": "value"
    },
    request_digest="request_digest",
    run_id="run_id",
    status="completed",
    tool_call_id="tool_call_id",
    tool_provider_id="tool_provider_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ReceiptInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_operations.<a href="src/sikaru_api/compute_operations/client.py">poll</a>(...) -> WorkPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_operations.poll(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**wait_seconds:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeWorkspaces
<details><summary><code>client.compute_workspaces.<a href="src/sikaru_api/compute_workspaces/client.py">record</a>(...) -> WorkspaceCheckpointView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record a verified uploaded manifest as this session's workspace checkpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_workspaces.record(
    project_id="project_id",
    attachment_id="attachment_id",
    manifest_digest="manifest_digest",
    trigger="turn",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**manifest_digest:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trigger:** `WorkspaceCheckpointInputTrigger` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_workspaces.<a href="src/sikaru_api/compute_workspaces/client.py">upload</a>(...) -> WorkspaceContentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload file or manifest bytes before acknowledging a workspace checkpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
client.compute_workspaces.upload(...)
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Union[bytes, typing.Iterator[bytes], typing.AsyncIterator[bytes]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_workspaces.<a href="src/sikaru_api/compute_workspaces/client.py">policy</a>(...) -> WorkspacePolicyView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read the pinned publication rules before capturing workspace files.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_workspaces.policy(
    project_id="project_id",
    attachment_id="attachment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeCredentials
<details><summary><code>client.compute_credentials.<a href="src/sikaru_api/compute_credentials/client.py">renew</a>(...) -> CredentialRenewed</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_credentials.renew(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_credentials.<a href="src/sikaru_api/compute_credentials/client.py">revoke</a>(...) -> CredentialRevoked</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_credentials.revoke(
    project_id="project_id",
    credential_id="credential_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**credential_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeEnvironments
<details><summary><code>client.compute_environments.<a href="src/sikaru_api/compute_environments/client.py">create</a>(...) -> EnvironmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Same key + same configuration returns the same environment; changed input conflicts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_environments.create(
    project_id="project_id",
    environment_slug="environment_slug",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_environments.<a href="src/sikaru_api/compute_environments/client.py">get</a>(...) -> EnvironmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_environments.get(
    project_id="project_id",
    environment_id="environment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.compute_environments.<a href="src/sikaru_api/compute_environments/client.py">disable</a>(...) -> EnvironmentView</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Idempotently disable future claims and credentials; existing cleanup remains required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.compute_environments.disable(
    project_id="project_id",
    environment_id="environment_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Connections
<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">list_connections</a>(...) -> typing.List[Connection]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.list_connections(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">create_connection</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ConnectionConfig
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.create_connection(
    project_id="project_id",
    config=ConnectionConfig(),
    display_name="display_name",
    kind="mcp",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**config:** `ConnectionConfig` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `CreateConnectionKind` 
    
</dd>
</dl>

<dl>
<dd>

**credentials:** `typing.Optional[ConnectionCredentials]` 
    
</dd>
</dl>

<dl>
<dd>

**ownership:** `typing.Optional[CreateConnectionOwnership]` 
    
</dd>
</dl>

<dl>
<dd>

**slug:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">list_apps</a>(...) -> ConnectionApps</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.list_apps(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**search:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**category:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">get_connection</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.get_connection(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">update_connection</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.update_connection(
    project_id="project_id",
    connection_id="connection_id",
    expected_version=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**expected_version:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**allowed_hosts:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">authorize</a>(...) -> ConnectionAuthorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.authorize(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">complete</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.complete(
    project_id="project_id",
    connection_id="connection_id",
    state="state",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CompleteAuthorization` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">credentials</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ConnectionCredentials
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.credentials(
    project_id="project_id",
    connection_id="connection_id",
    credentials=ConnectionCredentials(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ReplaceCredentials` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">disable</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.disable(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">disconnect</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.disconnect(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">discover</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry loading tools. Tools load automatically after sign-in and on creation; the outcome is recorded in `tool_load`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.discover(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">enable</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.enable(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">events</a>(...) -> ConnectionEventPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.events(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">grant</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.grant(
    project_id="project_id",
    connection_id="connection_id",
    agent_id="agent_id",
    tools=[
        "tools"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**approval:** `typing.Optional[GrantConnectionApproval]` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[GrantConnectionEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">revoke_grant</a>(...) -> Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.revoke_grant(
    project_id="project_id",
    connection_id="connection_id",
    grant_id="grant_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**grant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">revoke</a>(...) -> ConnectionRevocation</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.revoke(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="src/sikaru_api/connections/client.py">usage</a>(...) -> ConnectionUsage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.connections.usage(
    project_id="project_id",
    connection_id="connection_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ContextRegistry
<details><summary><code>client.context_registry.<a href="src/sikaru_api/context_registry/client.py">create_context_registry_change</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.context_registry.create_context_registry_change(
    project_id="project_id",
    name="name",
    repo_id="repoId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**repo_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**action:** `typing.Optional[CreateContextRegistryChangeRequestAction]` 
    
</dd>
</dl>

<dl>
<dd>

**files:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**linked_signal:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**owner:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**production_commit:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**production_tag:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**repo_type:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**source_trace_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**staging_commit:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**staging_tag:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**variables:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Conversations
<details><summary><code>client.conversations.<a href="src/sikaru_api/conversations/client.py">list_messages</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.conversations.list_messages(
    project_id="project_id",
    conversation_id="conversation_id",
    account_id="account_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="src/sikaru_api/conversations/client.py">record_message</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment
import datetime

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.conversations.record_message(
    project_id="project_id",
    conversation_id="conversation_id",
    account_id="account_id",
    content="content",
    delivered_at=datetime.datetime.fromisoformat("2024-01-15T09:30:00+00:00"),
    message_id="messageId",
    position=1,
    role="user",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**delivered_at:** `datetime.datetime` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**position:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `DeliveredMessageRole` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[DeliveredMessageEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CreditTopUps
<details><summary><code>client.credit_top_ups.<a href="src/sikaru_api/credit_top_ups/client.py">create</a>(...) -> CreditTopUp</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Offer a credit top-up for this project's organization; an agent pays `payUrl` from its own wallet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.credit_top_ups.create(
    project_id="project_id",
    amount_usd="amountUsd",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**amount_usd:** `str` — Whole cents, for example "20.00".
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.credit_top_ups.<a href="src/sikaru_api/credit_top_ups/client.py">get</a>(...) -> CreditTopUp</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read a top-up offer's status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.credit_top_ups.get(
    project_id="project_id",
    top_up_id="top_up_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**top_up_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Datasets
<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">list_datasets</a>(...) -> DatasetList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.list_datasets(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `typing.Optional[ListDatasetsDatasetsRequestPurpose]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">create_dataset</a>(...) -> DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.create_dataset(
    project_id="project_id",
    name="name",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `typing.Optional[CreateDatasetPurpose]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">capture_into_new_dataset</a>(...) -> BatchResult</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates the named dataset and adds the runs to it; a retry of the key reuses both.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, NewDataset, CaptureItem
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.capture_into_new_dataset(
    project_id="project_id",
    dataset=NewDataset(
        name="name",
    ),
    idempotency_key="idempotency_key",
    items=[
        CaptureItem(
            run_id="run_id",
        )
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset:** `NewDataset` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `typing.List[CaptureItem]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">get_dataset</a>(...) -> DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.get_dataset(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">delete_dataset</a>(...)</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.delete_dataset(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">update_dataset</a>(...) -> DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.update_dataset(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `typing.Optional[UpdateDatasetPurpose]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">capture_into_dataset</a>(...) -> BatchResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, CaptureItem
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.capture_into_dataset(
    project_id="project_id",
    dataset_id="dataset_id",
    idempotency_key="idempotency_key",
    items=[
        CaptureItem(
            run_id="run_id",
        )
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `typing.List[CaptureItem]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">start_dataset_checks</a>(...) -> DatasetChecksStarted</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

One Check per eligible example, each started as a production run of the agent's live version.

Credits are admitted before anything is created. A retry of the same key
returns the same checks and starts only what did not start before.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.start_dataset_checks(
    project_id="project_id",
    dataset_id="dataset_id",
    agent_slug="agent_slug",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_version:** `typing.Optional[int]` — Omit for the latest dataset version.
    
</dd>
</dl>

<dl>
<dd>

**version:** `typing.Optional[str]` — The agent version to check; Checks run on the agent's live version.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">preview_dataset_checks</a>(...) -> DatasetChecksPreview</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

How many examples can run as Checks, why the rest cannot, and the estimated model cost on ``agent``.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.preview_dataset_checks(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**version:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">list_dataset_check_results</a>(...) -> DatasetCheckResults</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pass rate per agent version for each dataset version run as Checks; failures name their example and run.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.list_dataset_check_results(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**version:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">list_examples</a>(...) -> DatasetExamplePage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.list_examples(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**version:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**offset:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">delete_example</a>(...) -> DatasetVersionResponse</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently removes every copy of the example, including from older versions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.delete_example(
    project_id="project_id",
    dataset_id="dataset_id",
    example_id="example_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**example_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">update_example</a>(...) -> DatasetExampleResponse</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Editing the expected answer or tags creates a version; edited fields are human-written.

Sikaru's own tags survive a tag edit unless it comes from the signed-in dashboard, which shows them.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.update_example(
    project_id="project_id",
    dataset_id="dataset_id",
    example_id="example_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**example_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**expected:** `typing.Optional[UpdateExampleExpected]` 
    
</dd>
</dl>

<dl>
<dd>

**tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">export_dataset</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.export_dataset(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**version:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">upload_examples</a>(...) -> BatchResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.upload_examples(
    project_id="project_id",
    dataset_id="dataset_id",
    content="content",
    format="csv",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `str` — CSV with input, expected and tags columns, or JSONL in the export shape.
    
</dd>
</dl>

<dl>
<dd>

**format:** `UploadRequestFormat` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="src/sikaru_api/datasets/client.py">list_versions</a>(...) -> DatasetVersionList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.datasets.list_versions(
    project_id="project_id",
    dataset_id="dataset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Deployments
<details><summary><code>client.deployments.<a href="src/sikaru_api/deployments/client.py">list_console_deployments</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.deployments.list_console_deployments(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Environments
<details><summary><code>client.environments.<a href="src/sikaru_api/environments/client.py">list_managed_environments</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.environments.list_managed_environments(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="src/sikaru_api/environments/client.py">create_managed_environment</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.environments.create_managed_environment(
    project_id="project_id",
    environment_slug="environmentSlug",
    provider_type="providerType",
    runtime_provider="runtimeProvider",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**provider_type:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**runtime_provider:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**config_refs:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationComparisons
<details><summary><code>client.evaluation_comparisons.<a href="src/sikaru_api/evaluation_comparisons/client.py">list_comparisons</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_comparisons.list_comparisons(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_comparisons.<a href="src/sikaru_api/evaluation_comparisons/client.py">create_comparison</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, CaseInput, AnswerRef
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_comparisons.create_comparison(
    project_id="project_id",
    cases=[
        CaseInput(
            baseline=AnswerRef(
                account_id="accountId",
                conversation_id="conversationId",
                message_id="messageId",
            ),
            candidate=AnswerRef(
                account_id="accountId",
                conversation_id="conversationId",
                message_id="messageId",
            ),
            case_id="caseId",
        )
    ],
    evaluator="evaluator",
    id="id",
    revision="revision",
    rubric="rubric",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cases:** `typing.List[CaseInput]` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**rubric:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_comparisons.<a href="src/sikaru_api/evaluation_comparisons/client.py">get_comparison</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_comparisons.get_comparison(
    project_id="project_id",
    comparison_id="comparison_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**comparison_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_comparisons.<a href="src/sikaru_api/evaluation_comparisons/client.py">cancel_comparison</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_comparisons.cancel_comparison(
    project_id="project_id",
    comparison_id="comparison_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**comparison_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationCriteria
<details><summary><code>client.evaluation_criteria.<a href="src/sikaru_api/evaluation_criteria/client.py">list_criteria</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_criteria.list_criteria(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationJobs
<details><summary><code>client.evaluation_jobs.<a href="src/sikaru_api/evaluation_jobs/client.py">list_jobs</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_jobs.list_jobs(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_jobs.<a href="src/sikaru_api/evaluation_jobs/client.py">create_job</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, TargetInput, JudgmentTarget
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_jobs.create_job(
    project_id="project_id",
    evaluator="evaluator",
    request_id="requestId",
    revision="revision",
    rubric="rubric",
    targets=[
        TargetInput(
            target=JudgmentTarget(
                account_id="accountId",
                kind="message",
            ),
            trace_id="traceId",
        )
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**rubric:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**targets:** `typing.List[TargetInput]` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[JobInputEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_jobs.<a href="src/sikaru_api/evaluation_jobs/client.py">get_job</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_jobs.get_job(
    project_id="project_id",
    job_id="job_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_jobs.<a href="src/sikaru_api/evaluation_jobs/client.py">cancel_job</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_jobs.cancel_job(
    project_id="project_id",
    job_id="job_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationResults
<details><summary><code>client.evaluation_results.<a href="src/sikaru_api/evaluation_results/client.py">list_results</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_results.list_results(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[ListResultsEvaluationResultsRequestEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**verdict:** `typing.Optional[ListResultsEvaluationResultsRequestVerdict]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluation_results.<a href="src/sikaru_api/evaluation_results/client.py">record_result</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluation_results.record_result(
    project_id="project_id",
    environment="production",
    evaluator="evaluator",
    id="id",
    public_reason="publicReason",
    revision="revision",
    source="human",
    trace_id="traceId",
    verdict="pass",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `EvaluationResultEnvironment` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**public_reason:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `EvaluationResultSource` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**verdict:** `EvaluationResultVerdict` 
    
</dd>
</dl>

<dl>
<dd>

**adjudicates:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**context:** `typing.Optional[JudgmentContext]` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `typing.Optional[JudgmentTarget]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluatorRuns
<details><summary><code>client.evaluator_runs.<a href="src/sikaru_api/evaluator_runs/client.py">create_evaluator_run</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.evaluator_runs.create_evaluator_run(
    project_id="project_id",
    trace_ids=[
        "traceIds"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**backfill_window:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**filter_expression:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_title:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sample_rate:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**signal_tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ExecutionObjectives
<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">list_objectives</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.list_objectives(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">create</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.create(
    project_id="project_id",
    criterion="criterion",
    idempotency_key="idempotency_key",
    objective="objective",
    run_id="run_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**criterion:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**evidence_paths:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**max_rounds:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">get</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.get(
    project_id="project_id",
    objective_id="objective_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">cancel</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.cancel(
    project_id="project_id",
    objective_id="objective_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">pause</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.pause(
    project_id="project_id",
    objective_id="objective_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_objectives.<a href="src/sikaru_api/execution_objectives/client.py">resume</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ResumeInput
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_objectives.resume(
    project_id="project_id",
    objective_id="objective_id",
    request=ResumeInput(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ResumeInput]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ExecutionSessions
<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">list</a>(...) -> ExecutionSessionPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.list(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">get</a>(...) -> ExecutionSessionResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.get(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">branch</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.branch(
    project_id="project_id",
    session_id="session_id",
    idempotency_key="idempotency_key",
    source_run_id="source_run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**source_run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">list_files</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.list_files(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">upload_file</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
client.execution_sessions.upload_file(...)
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**filename:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Union[bytes, typing.Iterator[bytes], typing.AsyncIterator[bytes]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">delete_file</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.delete_file(
    project_id="project_id",
    session_id="session_id",
    file_id="file_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">download_file</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.download_file(
    project_id="project_id",
    session_id="session_id",
    file_id="file_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">list_session_inputs</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.list_session_inputs(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">spend</a>(...) -> SessionSpend</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.spend(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">append_turn</a>(...) -> ExecutionTurnResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.append_turn(
    project_id="project_id",
    session_id="session_id",
    idempotency_key="idempotency_key",
    input={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**capability_grants:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**compute_attachment_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**delivery_mode:** `typing.Optional[TurnInputDeliveryMode]` 
    
</dd>
</dl>

<dl>
<dd>

**file_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**pinned_generation_id:** `typing.Optional[str]` — Set a release pin for new turns; explicit null resumes following promotions. Omit to preserve policy.
    
</dd>
</dl>

<dl>
<dd>

**product_context:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**run_mode:** `typing.Optional[TurnInputRunMode]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_provider_refs:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">create</a>(...) -> ExecutionSessionResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.create(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**acknowledge_widening:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**auto_improve:** `typing.Optional[bool]` — Automatically request evaluated harness improvements after completed turns. Requires harness:write and configured improvement policy.
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**draft_revision:** `typing.Optional[int]` — Saved document revision to test. Required for document Draft sessions.
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[SessionInputEnvironment]` — Draft sessions test the pinned agent definition without activation. Creating or appending draft sessions also requires harness:write.
    
</dd>
</dl>

<dl>
<dd>

**expected_access_digest:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**final_output_schema:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `typing.Optional[str]` — Default model for this session. Use a Sikaru model catalog ID, such as moonshotai/kimi-k3. Omit to inherit the project default.
    
</dd>
</dl>

<dl>
<dd>

**pinned_generation_id:** `typing.Optional[str]` — Optional previously serving release generation. Omit to follow promotions.
    
</dd>
</dl>

<dl>
<dd>

**reasoning_effort:** `typing.Optional[SessionInputReasoningEffort]` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.execution_sessions.<a href="src/sikaru_api/execution_sessions/client.py">list_eligible_releases</a>(...) -> EligibleReleases</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List release identities eligible for an optional session pin.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.execution_sessions.list_eligible_releases(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[ListEligibleReleasesExecutionSessionsRequestEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Specialists
<details><summary><code>client.specialists.<a href="src/sikaru_api/specialists/client.py">list</a>(...) -> SpecialistThreads</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.specialists.list(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="src/sikaru_api/specialists/client.py">get</a>(...) -> SpecialistThread</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.specialists.get(
    project_id="project_id",
    session_id="session_id",
    thread_id="thread_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="src/sikaru_api/specialists/client.py">cancel</a>(...) -> SpecialistReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.specialists.cancel(
    project_id="project_id",
    session_id="session_id",
    thread_id="thread_id",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="src/sikaru_api/specialists/client.py">message</a>(...) -> SpecialistReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.specialists.message(
    project_id="project_id",
    session_id="session_id",
    thread_id="thread_id",
    idempotency_key="idempotency_key",
    input={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**delivery_mode:** `typing.Optional[SpecialistMessageDeliveryMode]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Executions
<details><summary><code>client.executions.<a href="src/sikaru_api/executions/client.py">execution_runtime_lineage</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.executions.execution_runtime_lineage(
    project_id="project_id",
    trace_id="trace_id",
    account_id="account_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**inference_after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Feedback
<details><summary><code>client.feedback.<a href="src/sikaru_api/feedback/client.py">create_feedback</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.feedback.create_feedback(
    project_id="project_id",
    kind="thumbs_up",
    target="trace",
    target_id="targetId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `CreateFeedbackRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `CreateFeedbackRequestTarget` 
    
</dd>
</dl>

<dl>
<dd>

**target_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**note:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**span_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**tag:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## GitCredentials
<details><summary><code>client.git_credentials.<a href="src/sikaru_api/git_credentials/client.py">list</a>(...) -> typing.List[GitCredential]</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The project's git credentials and their grants, without values.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.git_credentials.list(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.git_credentials.<a href="src/sikaru_api/git_credentials/client.py">create</a>(...) -> GitCredential</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Store a git credential for one host. The value is write-only.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.git_credentials.create(
    project_id="project_id",
    host="host",
    token="token",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**host:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**token:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**username:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.git_credentials.<a href="src/sikaru_api/git_credentials/client.py">grant</a>(...) -> GitCredential</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Let an agent use a git credential for its declared repos.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.git_credentials.grant(
    project_id="project_id",
    credential_id="credential_id",
    agent_id="agentId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**credential_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Harnesses
<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">get_invoice_budget</a>(...) -> InvoiceBudget</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.get_invoice_budget(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">improvement_options</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.improvement_options(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">list_improvements</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.list_improvements(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">start_improvement</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.start_improvement(
    project_id="project_id",
    harness_id="harness_id",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**objective:** `typing.Optional[ImprovementInputObjective]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">get_improvement</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.get_improvement(
    project_id="project_id",
    harness_id="harness_id",
    job_id="job_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">resume_improvement</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ResumeImprovementInput
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.resume_improvement(
    project_id="project_id",
    harness_id="harness_id",
    job_id="job_id",
    request=ResumeImprovementInput(),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Optional[ResumeImprovementInput]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="src/sikaru_api/harnesses/client.py">train_model_stub</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Reserved, unavailable model-training step; no learning job is submitted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.harnesses.train_model_stub(
    project_id="project_id",
    harness_id="harness_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Runs
<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">start</a>(...) -> ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.start(
    project_id="project_id",
    harness_id="harness_id",
    input={
        "key": "value"
    },
    policy={
        "key": "value"
    },
    product_context={
        "key": "value"
    },
    tenant_id="tenant_id",
    user_id="user_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**policy:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**product_context:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**auto_improve:** `typing.Optional[bool]` — Automatically request evaluated harness improvements after completed turns. Requires harness:write and configured improvement policy; active runs keep their pinned release.
    
</dd>
</dl>

<dl>
<dd>

**capability_grants:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**compute_environment_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**compute_workspace_provenance:** `typing.Optional[WorkspaceProvenance]` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**correlation_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**event_delivery:** `typing.Optional[EventDeliveryRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `typing.Optional[str]` — Catalog model for this run. Omit to use the project default; list choices through model settings.
    
</dd>
</dl>

<dl>
<dd>

**run_mode:** `typing.Optional[StartHarnessRunRequestRunMode]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_provider_refs:** `typing.Optional[typing.List[ToolProviderRefRequest]]` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">record</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record input and output, optionally deduplicating retries with Idempotency-Key.

Reusing a key with different content returns 409. No execution timing is
inferred; the run is ordered by when Sikaru receives it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.record(
    project_id="project_id",
    input="input",
    output="output",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**output:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">resume_all</a>(...) -> ResumeResults</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Recheck up to 100 authorized needs-action runs and jobs; only cleared stops resume.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.resume_all(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `typing.Optional[ResumeFilterReason]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">resume_count</a>(...) -> ResumeCounts</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Count needs-action work the caller can control, grouped by funding reason.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.resume_count(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">get</a>(...) -> ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.get(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">pending_actions</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.pending_actions(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">cancel</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.cancel(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">events</a>(...) -> RunEvents</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.events(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">stream_events</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replay and stream durable run events; resume using after or Last-Event-ID.

A terminal run event ends the stream. Idle streams may also close: callers
can resume from their last delivered event without restarting the run.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.stream_events(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**last_event_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">recover</a>(...) -> ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.recover(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `RecoverRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">resume</a>(...) -> ResumeResult</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resume the same retained turn. A refused request does not change its state.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.resume(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">retry</a>(...) -> ExecutionTurnResponse</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry a failed turn as a new turn with its own run ID; the failed run stays failed. The retry
starts from the session's last completed turn, and external effects the failed turn already
completed are not repeated. Repeating the request returns the same retry. A run that did not
fail, whose `error.recoverable` is false, or that failed too long ago is refused with 409
`not_retryable`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.retry(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">decide_approval</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.decide_approval(
    project_id="project_id",
    run_id="run_id",
    tool_call_id="tool_call_id",
    decision="approved",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tool_call_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `ApprovalInput` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">submit_tool_result</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.submit_tool_result(
    project_id="project_id",
    run_id="run_id",
    capability_name="capability_name",
    idempotency_key="idempotency_key",
    payload={
        "key": "value"
    },
    status="completed",
    tool_call_id="tool_call_id",
    tool_provider_id="tool_provider_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**capability_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**payload:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `SubmitToolResultRequestStatus` 
    
</dd>
</dl>

<dl>
<dd>

**tool_call_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tool_provider_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">get_trajectory</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read retained ATIF structure and usage with private content redacted.

This is a committed snapshot and can be partial while a run is active or
interrupted. Messages, reasoning, tool payloads and provider metadata are
omitted. No trajectory is synthesized when retained evidence is unavailable.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.get_trajectory(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">get_transcript</a>(...) -> RunTranscript</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.get_transcript(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="src/sikaru_api/runs/client.py">stream_transcript_events</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.runs.stream_transcript_events(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**last_event_id:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ImportSessions
<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">list_import_sessions</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.list_import_sessions(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">create_import_session</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.create_import_session(
    project_id="project_id",
    display_name="displayName",
    mode="workflow",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**mode:** `CreateImportSessionRequestMode` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">get_import_session</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.get_import_session(
    project_id="project_id",
    import_session_id="import_session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">create_compatibility_profile</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.create_compatibility_profile(
    project_id="project_id",
    import_session_id="import_session_id",
    display_name="displayName",
    harness_id="harnessId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**approval_map:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**eval_map:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**input_schema:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**memory_map:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**output_schema:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**run_modes:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_map:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**trace_coverage_summary:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">list_import_session_diffs</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.list_import_session_diffs(
    project_id="project_id",
    import_session_id="import_session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">get_parity_report</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.get_parity_report(
    project_id="project_id",
    import_session_id="import_session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">promote_import_session</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.promote_import_session(
    project_id="project_id",
    import_session_id="import_session_id",
    active_harness_version_id="activeHarnessVersionId",
    agent_slug="agentSlug",
    compatibility_profile_id="compatibilityProfileId",
    display_name="displayName",
    harness_id="harnessId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**active_harness_version_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**compatibility_profile_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">create_replay_run</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.create_replay_run(
    project_id="project_id",
    import_session_id="import_session_id",
    compatibility_profile_id="compatibilityProfileId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CreateParityRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">list_source_artifacts</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.list_source_artifacts(
    project_id="project_id",
    import_session_id="import_session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">create_source_artifact</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.create_source_artifact(
    project_id="project_id",
    import_session_id="import_session_id",
    content_digest="contentDigest",
    export_policy="customer_source_exportable",
    source_kind="workflow_json",
    storage_ref="storageRef",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content_digest:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**export_policy:** `CreateSourceArtifactRequestExportPolicy` 
    
</dd>
</dl>

<dl>
<dd>

**source_kind:** `CreateSourceArtifactRequestSourceKind` 
    
</dd>
</dl>

<dl>
<dd>

**storage_ref:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**analysis_summary:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import_sessions.<a href="src/sikaru_api/import_sessions/client.py">create_staging_run</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.import_sessions.create_staging_run(
    project_id="project_id",
    import_session_id="import_session_id",
    compatibility_profile_id="compatibilityProfileId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `CreateParityRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## IssueClusters
<details><summary><code>client.issue_clusters.<a href="src/sikaru_api/issue_clusters/client.py">list_issue_clusters</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.issue_clusters.list_issue_clusters(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**severity:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issue_clusters.<a href="src/sikaru_api/issue_clusters/client.py">upsert_issue_cluster</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.issue_clusters.upsert_issue_cluster(
    project_id="project_id",
    label="label",
    trace_ids=[
        "traceIds"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**impact:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_title:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**signal_tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issue_clusters.<a href="src/sikaru_api/issue_clusters/client.py">get_issue_cluster</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.issue_clusters.get_issue_cluster(
    project_id="project_id",
    cluster_id="cluster_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issue_clusters.<a href="src/sikaru_api/issue_clusters/client.py">update_issue_cluster_status</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.issue_clusters.update_issue_cluster_status(
    project_id="project_id",
    cluster_id="cluster_id",
    status="status",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issue_clusters.<a href="src/sikaru_api/issue_clusters/client.py">propose_issue_cluster_fix</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.issue_clusters.propose_issue_cluster_fix(
    project_id="project_id",
    cluster_id="cluster_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## JudgeAlignment
<details><summary><code>client.judge_alignment.<a href="src/sikaru_api/judge_alignment/client.py">get_judge_alignment</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.judge_alignment.get_judge_alignment(
    project_id="project_id",
    evaluator="evaluator",
    revision="revision",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[GetJudgeAlignmentJudgeAlignmentRequestEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ManagedAgents
<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">list_managed_agents</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.list_managed_agents(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">create_managed_agent</a>(...) -> CreatedManagedAgent</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.create_managed_agent(
    project_id="project_id",
    agent_slug="agentSlug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**active_harness_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**background_turn_share:** `typing.Optional[float]` — Share of maxConcurrentTurns that background turns may use; omit to keep the current share.
    
</dd>
</dl>

<dl>
<dd>

**compatibility_profile_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**end_user_cells:** `typing.Optional[bool]` — Fixed at creation. Each end user (tenantId, userId) of the agent gets a private cell: their memory, corrections and learning stay theirs. Omit it to keep an existing agent's choice.
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**initial_channel:** `typing.Optional[CreateManagedAgentRequestInitialChannel]` 
    
</dd>
</dl>

<dl>
<dd>

**max_concurrent_turns:** `typing.Optional[int]` — Most turns the agent runs at once; omit to keep the current limit.
    
</dd>
</dl>

<dl>
<dd>

**personal:** `typing.Optional[bool]` — Fixed at creation; new agents are shared unless set. A personal agent's conversations stay private to the person who runs them. Omit it to keep an existing agent's choice.
    
</dd>
</dl>

<dl>
<dd>

**source:** `typing.Optional[CreateManagedAgentSourceRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[CreateManagedAgentRequestStatus]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">get</a>(...) -> ManagedAgentRead</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

One managed agent in the project: the same public summary the list returns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.get(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">delete_managed_agent</a>(...)</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an agent that has never gone live, with its drafts, checks and schedules.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.delete_managed_agent(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">update</a>(...) -> RenamedManagedAgent</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Change the agent's display name or turn capacity. The slug and every reference to the agent stay the same.

Personal and end-user-cell choices are fixed at creation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.update(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**background_turn_share:** `typing.Optional[float]` — Share of maxConcurrentTurns that background turns may use; null restores the default.
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**max_concurrent_turns:** `typing.Optional[int]` — Most turns the agent runs at once; null restores the default.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">create_definition_revision</a>(...) -> DefinitionRevisionResult</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Stage a changed definition as a draft revision; the live definition is a no-op.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, AgentDefinition
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.create_definition_revision(
    project_id="project_id",
    agent_slug="agent_slug",
    content_digest="contentDigest",
    definition=AgentDefinition(
        schema="sikaru.agent.contract.v1",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content_digest:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**definition:** `AgentDefinition` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managed_agents.<a href="src/sikaru_api/managed_agents/client.py">get_definition_revision</a>(...) -> DefinitionRevisionView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.managed_agents.get_definition_revision(
    project_id="project_id",
    agent_slug="agent_slug",
    changeset_id="changeset_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Checks
<details><summary><code>client.checks.<a href="src/sikaru_api/checks/client.py">list</a>(...) -> CheckList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.checks.list(
    project_id="project_id",
    agent_slug="agent_slug",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="src/sikaru_api/checks/client.py">create</a>(...) -> CheckResponse</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Same key and definition return the same check; a changed definition conflicts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, CheckEnvironment, HarborTaskFiles
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.checks.create(
    project_id="project_id",
    agent_slug="agent_slug",
    environment=CheckEnvironment(
        kind="managed",
    ),
    idempotency_key="idempotency_key",
    name="name",
    task=HarborTaskFiles(
        files={
            "key": "value"
        },
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `CheckEnvironment` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**task:** `HarborTaskFiles` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="src/sikaru_api/checks/client.py">list_results</a>(...) -> CheckResultList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.checks.list_results(
    project_id="project_id",
    agent_slug="agent_slug",
    check_id="check_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**check_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="src/sikaru_api/checks/client.py">run</a>(...) -> CheckResultResponse</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Start the check's task as a real run of the agent's active release; the result settles later.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.checks.run(
    project_id="project_id",
    agent_slug="agent_slug",
    check_id="check_id",
    idempotency_key="idempotency_key",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**check_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EndUsers
<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">get</a>(...) -> EndUser</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

What the agent knows about this end user: facts and the persona summary, never the persona prompt.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.get(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">upsert</a>(...) -> EndUser</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create this end user's private cell, or update its timezone and population opt-in.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.upsert(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**opt_in:** `typing.Optional[bool]` — Opting out removes this user's contributed traces and eval cases from population material. Omit it to keep the current choice.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `typing.Optional[str]` — IANA timezone, such as Europe/Berlin. Explicit null clears it; omit it to keep the current value.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">delete</a>(...) -> EndUserDeletion</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Purge this user from every agent of the product, with every trace or eval case they contributed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.delete(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">update_fact</a>(...) -> EndUserFile</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Correct or add one fact. It serves from the next turn and becomes a held-out case for this user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.update_fact(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
    content="content",
    path="path",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**path:** `str` — USER.md or memories/<name>.md.
    
</dd>
</dl>

<dl>
<dd>

**expected_etag:** `typing.Optional[str]` — Refuse with 409 unless the fact still has this etag.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">delete_fact</a>(...)</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Forget one memory, or clear the profile.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.delete_fact(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
    path="path",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**path:** `str` — USER.md or memories/<name>.md.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.end_users.<a href="src/sikaru_api/end_users/client.py">correct_persona_summary</a>(...) -> RecordedCorrection</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Correct the persona summary. The correction applies from the next turn and is never relearned away.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.end_users.correct_persona_summary(
    project_id="project_id",
    agent_slug="agent_slug",
    tenant_id="tenant_id",
    user_id="user_id",
    text="text",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `str` — What the summary gets wrong, in the user's words.
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## MemoryRegistry
<details><summary><code>client.memory_registry.<a href="src/sikaru_api/memory_registry/client.py">create_memory_registry_change</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.memory_registry.create_memory_registry_change(
    project_id="project_id",
    memory_id="memoryId",
    memory_type="memoryType",
    name="name",
    scope="scope",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**memory_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**memory_type:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**action:** `typing.Optional[CreateMemoryRegistryChangeRequestAction]` 
    
</dd>
</dl>

<dl>
<dd>

**candidate_release_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**confidence:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**policy:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**provenance:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**source_trace_ids:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**ttl:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ModelGateway
<details><summary><code>client.model_gateway.<a href="src/sikaru_api/model_gateway/client.py">capture_model_gateway_chat_completion</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.model_gateway.capture_model_gateway_chat_completion(
    project_id="project_id",
    provider="provider",
    request={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**error:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**response:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ModelSettings
<details><summary><code>client.model_settings.<a href="src/sikaru_api/model_settings/client.py">get_model_settings</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.model_settings.get_model_settings(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.model_settings.<a href="src/sikaru_api/model_settings/client.py">update_model_settings</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.model_settings.update_model_settings(
    project_id="project_id",
    selected_model_id="selectedModelId",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**selected_model_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## OnlineEvaluations
<details><summary><code>client.online_evaluations.<a href="src/sikaru_api/online_evaluations/client.py">list_policies</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.online_evaluations.list_policies(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.online_evaluations.<a href="src/sikaru_api/online_evaluations/client.py">create_policy</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.online_evaluations.create_policy(
    project_id="project_id",
    evaluator="evaluator",
    id="id",
    revision="revision",
    rubric="rubric",
    sample_percent=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**revision:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**rubric:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**sample_percent:** `int` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[PolicyInputEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.online_evaluations.<a href="src/sikaru_api/online_evaluations/client.py">preview_policy_eligibility</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.online_evaluations.preview_policy_eligibility(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[PreviewPolicyEligibilityOnlineEvaluationsRequestEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.online_evaluations.<a href="src/sikaru_api/online_evaluations/client.py">update_policy</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.online_evaluations.update_policy(
    project_id="project_id",
    policy_id="policy_id",
    enabled=True,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**policy_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ReleaseWatches
<details><summary><code>client.release_watches.<a href="src/sikaru_api/release_watches/client.py">create_release_watch</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.release_watches.create_release_watch(
    project_id="project_id",
    trace_ids=[
        "traceIds"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**backfill_window:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**filter_expression:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_title:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**release_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sample_rate:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**signal_tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RetentionPolicies
<details><summary><code>client.retention_policies.<a href="src/sikaru_api/retention_policies/client.py">create_retention_policy_update</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.retention_policies.create_retention_policy_update(
    project_id="project_id",
    trace_ids=[
        "traceIds"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**backfill_window:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**filter_expression:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_title:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**retention_days:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sample_rate:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**signal_tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ReviewQueue
<details><summary><code>client.review_queue.<a href="src/sikaru_api/review_queue/client.py">create_review_queue_item</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.review_queue.create_review_queue_item(
    project_id="project_id",
    trace_ids=[
        "traceIds"
    ],
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**action:** `typing.Optional[CreateReviewQueueItemRequestAction]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**backfill_window:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**filter_expression:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**issue_title:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**rule_name:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sample_rate:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**signal_tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunReferences
<details><summary><code>client.run_references.<a href="src/sikaru_api/run_references/client.py">resolve_run_reference</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_references.resolve_run_reference(
    project_id="project_id",
    reference="reference",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**as_of:** `typing.Optional[datetime.datetime]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunSchedules
<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">list_schedules</a>(...) -> ScheduleList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.list_schedules(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">create_schedule</a>(...) -> ScheduleResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.create_schedule(
    project_id="project_id",
    input={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**cron:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `typing.Optional[ScheduleInputEnvironment]` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**interval_seconds:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**session_mode:** `typing.Optional[ScheduleInputSessionMode]` 
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">delete_schedule</a>(...) -> ScheduleDeleted</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.delete_schedule(
    project_id="project_id",
    schedule_id="schedule_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">pause_schedule</a>(...) -> SchedulePaused</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.pause_schedule(
    project_id="project_id",
    schedule_id="schedule_id",
    paused=True,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**paused:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">schedule_notices</a>(...) -> ScheduleNotices</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.schedule_notices(
    project_id="project_id",
    schedule_id="schedule_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_schedules.<a href="src/sikaru_api/run_schedules/client.py">schedule_history</a>(...) -> ScheduleHistory</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_schedules.schedule_history(
    project_id="project_id",
    schedule_id="schedule_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunWebhooks
<details><summary><code>client.run_webhooks.<a href="src/sikaru_api/run_webhooks/client.py">list_webhooks</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_webhooks.list_webhooks(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_webhooks.<a href="src/sikaru_api/run_webhooks/client.py">create_webhook</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_webhooks.create_webhook(
    project_id="project_id",
    run_id="run_id",
    url="url",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.run_webhooks.<a href="src/sikaru_api/run_webhooks/client.py">delete_webhook</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.run_webhooks.delete_webhook(
    project_id="project_id",
    webhook_id="webhook_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Sessions
<details><summary><code>client.sessions.<a href="src/sikaru_api/sessions/client.py">get_managed_session</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.sessions.get_managed_session(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="src/sikaru_api/sessions/client.py">list_managed_session_events</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.sessions.list_managed_session_events(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="src/sikaru_api/sessions/client.py">list_managed_session_files</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.sessions.list_managed_session_files(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="src/sikaru_api/sessions/client.py">list_managed_session_plan</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.sessions.list_managed_session_plan(
    project_id="project_id",
    session_id="session_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ToolProviders
<details><summary><code>client.tool_providers.<a href="src/sikaru_api/tool_providers/client.py">register_tool_provider</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.tool_providers.register_tool_provider(
    project_id="project_id",
    broker_endpoint_ref="broker_endpoint_ref",
    capability_prefix="capability_prefix",
    display_name="display_name",
    provider_type="provider_type",
    tool_catalog_ref="tool_catalog_ref",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**broker_endpoint_ref:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**capability_prefix:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**provider_type:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tool_catalog_ref:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**approval_policy_ref:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**credential_owner:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tool_providers.<a href="src/sikaru_api/tool_providers/client.py">attach_source_tool_skill</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, ToolSkillSourceRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.tool_providers.attach_source_tool_skill(
    project_id="project_id",
    tool_provider_id="tool_provider_id",
    capability_refs=[
        "capability_refs"
    ],
    description="description",
    source=ToolSkillSourceRequest(
        kind="kind",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**tool_provider_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**capability_refs:** `typing.List[str]` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `ToolSkillSourceRequest` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceImportConnections
<details><summary><code>client.trace_import_connections.<a href="src/sikaru_api/trace_import_connections/client.py">list_trace_import_connections</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_import_connections.list_trace_import_connections(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceImports
<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">list_trace_imports</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.list_trace_imports(
    project_id="project_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">create_trace_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, TraceImportScopeRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.create_trace_import(
    project_id="project_id",
    connection_id="connectionId",
    converter_version="converterVersion",
    dataset="dataset",
    external_project_id="externalProjectId",
    mode="historical",
    provider="langsmith",
    scope=TraceImportScopeRequest(),
    source_instance="sourceInstance",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**converter_version:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**external_project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**mode:** `CreateTraceImportRequestMode` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `CreateTraceImportRequestProvider` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `TraceImportScopeRequest` 
    
</dd>
</dl>

<dl>
<dd>

**source_instance:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**grouping:** `typing.Optional[TraceImportGroupingRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `typing.Optional[TraceImportOptionsRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">plan_trace_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, TraceImportScopeRequest
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.plan_trace_import(
    project_id="project_id",
    connection_id="connectionId",
    converter_version="converterVersion",
    dataset="dataset",
    external_project_id="externalProjectId",
    mode="historical",
    provider="langsmith",
    scope=TraceImportScopeRequest(),
    source_instance="sourceInstance",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**converter_version:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**dataset:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**external_project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**mode:** `PlanTraceImportRequestMode` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `PlanTraceImportRequestProvider` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `TraceImportScopeRequest` 
    
</dd>
</dl>

<dl>
<dd>

**source_instance:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**grouping:** `typing.Optional[TraceImportGroupingRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `typing.Optional[TraceImportOptionsRequest]` 
    
</dd>
</dl>

<dl>
<dd>

**tags:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**window_duration_hours:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">get_trace_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.get_trace_import(
    project_id="project_id",
    trace_import_id="trace_import_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">cancel_trace_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.cancel_trace_import(
    project_id="project_id",
    trace_import_id="trace_import_id",
    expected_version=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `VersionedTraceImportAction` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">get_trace_import_receipt</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.get_trace_import_receipt(
    project_id="project_id",
    trace_import_id="trace_import_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.trace_imports.<a href="src/sikaru_api/trace_imports/client.py">retry_trace_import</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_imports.retry_trace_import(
    project_id="project_id",
    trace_import_id="trace_import_id",
    expected_version=1,
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `VersionedTraceImportAction` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## WorkflowIntents
<details><summary><code>client.workflow_intents.<a href="src/sikaru_api/workflow_intents/client.py">create_project_workflow_intent</a>(...) -> typing.Dict[str, str]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflow_intents.create_project_workflow_intent(
    project_id="project_id",
    description="description",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflow_intents.<a href="src/sikaru_api/workflow_intents/client.py">compile_project_workflow_intent</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflow_intents.compile_project_workflow_intent(
    project_id="project_id",
    intent_id="intent_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**intent_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## WorkflowRuns
<details><summary><code>client.workflow_runs.<a href="src/sikaru_api/workflow_runs/client.py">project_workflow_run_events</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflow_runs.project_workflow_run_events(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflow_runs.<a href="src/sikaru_api/workflow_runs/client.py">recover_project_workflow_run</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflow_runs.recover_project_workflow_run(
    project_id="project_id",
    run_id="run_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `RecoverRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Workflows
<details><summary><code>client.workflows.<a href="src/sikaru_api/workflows/client.py">import_workflow</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflows.import_workflow(
    project_id="project_id",
    payload={
        "key": "value"
    },
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**payload:** `typing.Dict[str, typing.Any]` 
    
</dd>
</dl>

<dl>
<dd>

**source_artifact_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="src/sikaru_api/workflows/client.py">export_product_workflow</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflows.export_product_workflow(
    project_id="project_id",
    workflow_id="workflow_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="src/sikaru_api/workflows/client.py">start_project_workflow_run</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflows.start_project_workflow_run(
    project_id="project_id",
    workflow_id="workflow_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_version_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="src/sikaru_api/workflows/client.py">create_project_workflow_version</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.workflows.create_project_workflow_version(
    project_id="project_id",
    workflow_id="workflow_id",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**release_channel:** `typing.Optional[CreateWorkflowVersionRequestReleaseChannel]` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `typing.Optional[CreateWorkflowVersionRequestStatus]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Responses
<details><summary><code>client.responses.<a href="src/sikaru_api/responses/client.py">create_stream</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.responses.create_stream(
    model="model",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**access_programs:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**background:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**context_management:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**conversation:** `typing.Optional[CreateResponsesStreamRequestConversation]` 
    
</dd>
</dl>

<dl>
<dd>

**include:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Optional[CreateResponsesStreamRequestInput]` 
    
</dd>
</dl>

<dl>
<dd>

**instructions:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**max_output_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tool_calls:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**moderation:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**parallel_tool_calls:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**previous_response_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_retention:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**safety_identifier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**service_tier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**store:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_choice:** `typing.Optional[CreateResponsesStreamRequestToolChoice]` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**top_logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**truncation:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.responses.<a href="src/sikaru_api/responses/client.py">create</a>(...) -> ModelResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.responses.create_stream(
    model="model",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**access_programs:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**background:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**context_management:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**conversation:** `typing.Optional[CreateResponsesRequestConversation]` 
    
</dd>
</dl>

<dl>
<dd>

**include:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**input:** `typing.Optional[CreateResponsesRequestInput]` 
    
</dd>
</dl>

<dl>
<dd>

**instructions:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**max_output_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tool_calls:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**moderation:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**parallel_tool_calls:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**previous_response_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_retention:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**safety_identifier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**service_tier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**store:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_choice:** `typing.Optional[CreateResponsesRequestToolChoice]` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**top_logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**truncation:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Decisions
<details><summary><code>client.decisions.<a href="src/sikaru_api/decisions/client.py">create</a>(...) -> Decision</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.decisions.create(
    questions={
        "key": {
            "key": "value"
        }
    },
    state="state",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**questions:** `typing.Dict[str, typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**state:** `SystemOneRequestState` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceStreams
<details><summary><code>client.trace_streams.<a href="src/sikaru_api/trace_streams/client.py">stream_openinference_spans</a>(...) -> typing.Dict[str, typing.Any]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi, TraceMetadata
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.trace_streams.stream_openinference_spans(
    format="openinference.v1",
    metadata=TraceMetadata(
        account_id="account_id",
        project_id="project_id",
        source="source",
    ),
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `TraceStreamRequestFormat` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `TraceMetadata` 
    
</dd>
</dl>

<dl>
<dd>

**idempotency_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**sikaru_client_id:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**dataset:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**resource_spans:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**spans:** `typing.Optional[typing.List[OpenInferenceSpan]]` 
    
</dd>
</dl>

<dl>
<dd>

**trajectories:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**trajectory:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Chat Completions
<details><summary><code>client.chat.completions.<a href="src/sikaru_api/chat/completions/client.py">create_stream</a>(...) -> typing.Iterator[bytes]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.chat.completions.create_stream(
    messages=[
        {
            "key": "value"
        }
    ],
    model="model",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**messages:** `typing.List[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**audio:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**function_call:** `typing.Optional[CreateCompletionsStreamRequestFunctionCall]` 
    
</dd>
</dl>

<dl>
<dd>

**functions:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**include_reasoning:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**logit_bias:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**logprobs:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**max_completion_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**min_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**modalities:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**moderation:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**n:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**parallel_tool_calls:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**prediction:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_retention:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_effort:** `typing.Optional[CreateCompletionsStreamRequestReasoningEffort]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_history:** `typing.Optional[CreateCompletionsStreamRequestReasoningHistory]` 
    
</dd>
</dl>

<dl>
<dd>

**repetition_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**response_format:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**safety_identifier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**seed:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**service_tier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**stop:** `typing.Optional[CreateCompletionsStreamRequestStop]` 
    
</dd>
</dl>

<dl>
<dd>

**store:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**thinking:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_choice:** `typing.Optional[CreateCompletionsStreamRequestToolChoice]` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**top_k:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**typical_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**verbosity:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**web_search_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.completions.<a href="src/sikaru_api/chat/completions/client.py">create</a>(...) -> ChatCompletion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```python
from sikaru_api import SikaruApi
from sikaru_api.environment import SikaruApiEnvironment

client = SikaruApi(
    api_key="<token>",
    environment=SikaruApiEnvironment.DEFAULT,
)

client.chat.completions.create_stream(
    messages=[
        {
            "key": "value"
        }
    ],
    model="model",
)

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**messages:** `typing.List[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `str` 
    
</dd>
</dl>

<dl>
<dd>

**stream:** `typing.Literal` 
    
</dd>
</dl>

<dl>
<dd>

**audio:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**function_call:** `typing.Optional[CreateCompletionsRequestFunctionCall]` 
    
</dd>
</dl>

<dl>
<dd>

**functions:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**include_reasoning:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**logit_bias:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**logprobs:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**max_completion_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**max_tokens:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**min_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**modalities:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**models:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**moderation:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**n:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**parallel_tool_calls:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**prediction:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_key:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_cache_retention:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_effort:** `typing.Optional[CreateCompletionsRequestReasoningEffort]` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_history:** `typing.Optional[CreateCompletionsRequestReasoningHistory]` 
    
</dd>
</dl>

<dl>
<dd>

**repetition_penalty:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**response_format:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**route:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**safety_identifier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**seed:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**service_tier:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**stop:** `typing.Optional[CreateCompletionsRequestStop]` 
    
</dd>
</dl>

<dl>
<dd>

**store:** `typing.Optional[bool]` 
    
</dd>
</dl>

<dl>
<dd>

**stream_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**thinking:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**tool_choice:** `typing.Optional[CreateCompletionsRequestToolChoice]` 
    
</dd>
</dl>

<dl>
<dd>

**tools:** `typing.Optional[typing.List[typing.Dict[str, typing.Any]]]` 
    
</dd>
</dl>

<dl>
<dd>

**top_k:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_logprobs:** `typing.Optional[int]` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**transforms:** `typing.Optional[typing.List[str]]` 
    
</dd>
</dl>

<dl>
<dd>

**typical_p:** `typing.Optional[float]` 
    
</dd>
</dl>

<dl>
<dd>

**usage:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**user:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**verbosity:** `typing.Optional[str]` 
    
</dd>
</dl>

<dl>
<dd>

**web_search_options:** `typing.Optional[typing.Dict[str, typing.Any]]` 
    
</dd>
</dl>

<dl>
<dd>

**request_options:** `typing.Optional[RequestOptions]` — Request-specific configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

