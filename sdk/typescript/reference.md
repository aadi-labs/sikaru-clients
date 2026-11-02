# Reference
## Auth
<details><summary><code>client.auth.<a href="/src/api/resources/auth/client/Client.ts">getDeviceConfiguration</a>() -> Sikaru.DeviceConfiguration</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.auth.getDeviceConfiguration();

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**requestOptions:** `AuthClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## HttpChannels
<details><summary><code>client.httpChannels.<a href="/src/api/resources/httpChannels/client/Client.ts">invoke</a>(binding_id, { ...params }) -> Sikaru.HttpReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.httpChannels.invoke("binding_id", {
    content: "content",
    conversation_id: "conversation_id",
    message_id: "message_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.HttpMessage` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HttpChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.httpChannels.<a href="/src/api/resources/httpChannels/client/Client.ts">poll</a>(binding_id, receipt_id, { ...params }) -> Sikaru.HttpReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.httpChannels.poll("binding_id", "receipt_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PollHttpChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HttpChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.httpChannels.<a href="/src/api/resources/httpChannels/client/Client.ts">startSlackLink</a>(binding_id, { ...params }) -> Sikaru.PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.httpChannels.startSlackLink("binding_id", {
    body: {
        installation_id: "installation_id"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartSlackLinkHttpChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HttpChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.httpChannels.<a href="/src/api/resources/httpChannels/client/Client.ts">unlinkSlackIdentity</a>(binding_id, installation_id, { ...params }) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.httpChannels.unlinkSlackIdentity("binding_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UnlinkSlackIdentityHttpChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HttpChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Completions
<details><summary><code>client.completions.<a href="/src/api/resources/completions/client/Client.ts">createStream</a>({ ...params }) -> core.Stream&lt;Sikaru.Completion&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
const response = await client.completions.createStream({
    model: "model",
    prompt: "prompt"
});
for await (const item of response) {
    console.log(item);
}

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.CompletionCreateParamsStreaming` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CompletionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.completions.<a href="/src/api/resources/completions/client/Client.ts">create</a>({ ...params }) -> Sikaru.Completion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.completions.create({
    model: "model",
    prompt: "prompt"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.CompletionCreateParams` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CompletionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Models
<details><summary><code>client.models.<a href="/src/api/resources/models/client/Client.ts">list</a>() -> Sikaru.ModelList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.models.list();

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**requestOptions:** `ModelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.models.<a href="/src/api/resources/models/client/Client.ts">retrieve</a>(model) -> Sikaru.PublicModel</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.models.retrieve("model");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ModelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PersonalChannels
<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">getMessage</a>(binding_id, receipt_id, { ...params }) -> Sikaru.PersonalChannelReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.getMessage("binding_id", "receipt_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.GetMessagePersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">authorizeConnection</a>(binding_id, receipt_id, connection_id, { ...params }) -> Sikaru.ConnectionAuthorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.authorizeConnection("binding_id", "receipt_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.AuthorizeConnectionPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">completeConnection</a>(binding_id, receipt_id, connection_id, { ...params }) -> Sikaru.PersonalChannelConnection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.completeConnection("binding_id", "receipt_id", "connection_id", {
    body: {
        state: "state"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CompleteConnectionPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">replaceConnectionCredentials</a>(binding_id, receipt_id, connection_id, { ...params }) -> Sikaru.PersonalChannelConnection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.replaceConnectionCredentials("binding_id", "receipt_id", "connection_id", {
    body: {
        credentials: {}
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReplaceConnectionCredentialsPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">listFiles</a>(binding_id, receipt_id, { ...params }) -> Sikaru.PersonalChannelFiles</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.listFiles("binding_id", "receipt_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListFilesPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">downloadFile</a>(binding_id, receipt_id, file_id, { ...params }) -> core.BinaryResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.downloadFile("binding_id", "receipt_id", "file_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DownloadFilePersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">decideApproval</a>(binding_id, receipt_id, tool_call_id, { ...params }) -> Sikaru.PersonalChannelApproval</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.decideApproval("binding_id", "receipt_id", "tool_call_id", {
    body: {
        decision: "approved",
        idempotency_key: "idempotency_key"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tool_call_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DecideApprovalPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">getSlackLink</a>(binding_id, verification_id, { ...params }) -> Sikaru.PersonalSlackStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.getSlackLink("binding_id", "verification_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.GetSlackLinkPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">startSlackLink</a>(binding_id, { ...params }) -> Sikaru.PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.startSlackLink("binding_id", {
    body: {
        installation_id: "installation_id"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartSlackLinkPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.personalChannels.<a href="/src/api/resources/personalChannels/client/Client.ts">unlinkSlackIdentity</a>(binding_id, installation_id, { ...params }) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.personalChannels.unlinkSlackIdentity("binding_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UnlinkSlackIdentityPersonalChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `PersonalChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Activation
<details><summary><code>client.activation.<a href="/src/api/resources/activation/client/Client.ts">projectActivationStatus</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.activation.projectActivationStatus("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ActivationClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AgentDocuments
<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">draft</a>(project_id, { ...params }) -> Sikaru.AgentDocumentDraft</code></summary>
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

```typescript
await client.agentDocuments.draft("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DraftDocumentInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">importFiles</a>(project_id, { ...params }) -> Sikaru.ImportedDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.importFiles("project_id", {
    files: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ImportDocumentFiles` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">listResources</a>(project_id) -> Sikaru.DocumentResources</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.listResources("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">editSetting</a>(project_id, { ...params }) -> Sikaru.ImportedDocument</code></summary>
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

```typescript
await client.agentDocuments.editSetting("project_id", {
    document: "document",
    path: ["path"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.EditDocumentSetting` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">listTemplates</a>(project_id) -> Sikaru.DocumentTemplates</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.listTemplates("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">validateText</a>(project_id, { ...params }) -> Sikaru.DocumentValidationView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.validateText("project_id", {
    document: "document"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DocumentInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">get</a>(project_id, agent_slug) -> Sikaru.AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.get("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">save</a>(project_id, agent_slug, { ...params }) -> Sikaru.AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.save("project_id", "agent_slug", {
    document: "document",
    expectedRevision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SaveDocument` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">compare</a>(project_id, agent_slug) -> Sikaru.DocumentComparison</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.compare("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">discard</a>(project_id, agent_slug, { ...params }) -> Sikaru.AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.discard("project_id", "agent_slug", {
    expectedRevision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DiscardDocument` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">publish</a>(project_id, agent_slug, { ...params }) -> Sikaru.DocumentPublication</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.publish("project_id", "agent_slug", {
    revision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PublishDocument` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">revert</a>(project_id, agent_slug, { ...params }) -> Sikaru.DocumentPublication</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.revert("project_id", "agent_slug", {
    harnessVersionId: "harnessVersionId",
    revision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RevertDocument` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">review</a>(project_id, agent_slug, { ...params }) -> Sikaru.DocumentReview</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.review("project_id", "agent_slug", {
    revision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReviewDocument` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">snippets</a>(project_id, agent_slug) -> Sikaru.AgentSnippets</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.snippets("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">listSuggestions</a>(project_id, agent_slug) -> Sikaru.DocumentSuggestions</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.listSuggestions("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">adoptSuggestion</a>(project_id, agent_slug, suggestion_id, { ...params }) -> Sikaru.AgentDocument</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.adoptSuggestion("project_id", "agent_slug", "suggestion_id", {
    expectedRevision: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**suggestion_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.AdoptSuggestion` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">dismissSuggestion</a>(project_id, agent_slug, suggestion_id) -> Sikaru.DocumentSuggestion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.dismissSuggestion("project_id", "agent_slug", "suggestion_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**suggestion_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">validate</a>(project_id, agent_slug, { ...params }) -> Sikaru.DocumentValidationView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.validate("project_id", "agent_slug", {
    document: "document"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DocumentInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentDocuments.<a href="/src/api/resources/agentDocuments/client/Client.ts">listVersions</a>(project_id, agent_slug) -> Sikaru.DocumentVersions</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentDocuments.listVersions("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentDocumentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AgentImports
<details><summary><code>client.agentImports.<a href="/src/api/resources/agentImports/client/Client.ts">listAgentImports</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentImports.listAgentImports("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agentImports.<a href="/src/api/resources/agentImports/client/Client.ts">createAgentImport</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agentImports.createAgentImport("project_id", {
    improve: {
        mode: "mode",
        objective: "objective",
        promotionGate: "promotionGate"
    },
    name: "name"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateAgentImportRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents
<details><summary><code>client.agents.<a href="/src/api/resources/agents/client/Client.ts">createManagedSession</a>(project_id, agent_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.agents.createManagedSession("project_id", "agent_id", {
    environmentId: "environmentId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateManagedSessionRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `AgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CapabilityCeilings
<details><summary><code>client.capabilityCeilings.<a href="/src/api/resources/capabilityCeilings/client/Client.ts">get</a>(project_id) -> Sikaru.CapabilityCeilingsView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.capabilityCeilings.get("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CapabilityCeilingsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capabilityCeilings.<a href="/src/api/resources/capabilityCeilings/client/Client.ts">update</a>(project_id, { ...params }) -> Sikaru.CapabilityCeilingsView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.capabilityCeilings.update("project_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CapabilityCeilings` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CapabilityCeilingsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Changesets
<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">listChangesets</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.listChangesets("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListChangesetsChangesetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">createChangeset</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.createChangeset("project_id", {
    publicSummary: "publicSummary",
    publicTitle: "publicTitle",
    scopeRef: "scopeRef",
    scopeType: "global",
    sourceType: "import"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateChangeSetRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">getChangeset</a>(project_id, changeset_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.getChangeset("project_id", "changeset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">approveChangeset</a>(project_id, changeset_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.approveChangeset("project_id", "changeset_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReleaseActionRequest | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">listChangesetDiffs</a>(project_id, changeset_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.listChangesetDiffs("project_id", "changeset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">listChangesetEvidence</a>(project_id, changeset_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.listChangesetEvidence("project_id", "changeset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">promoteChangeset</a>(project_id, changeset_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.promoteChangeset("project_id", "changeset_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReleaseActionRequest | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">rejectChangeset</a>(project_id, changeset_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.rejectChangeset("project_id", "changeset_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReleaseActionRequest | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">rollbackChangeset</a>(project_id, changeset_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.rollbackChangeset("project_id", "changeset_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReleaseActionRequest | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.changesets.<a href="/src/api/resources/changesets/client/Client.ts">stageChangeset</a>(project_id, changeset_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.changesets.stageChangeset("project_id", "changeset_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReleaseActionRequest | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChangesetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Channels
<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">availability</a>(project_id) -> Sikaru.Availability</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.availability("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">bindings</a>(project_id, { ...params }) -> Sikaru.Binding[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.bindings("project_id", {
    agent_id: "agent_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.BindingsChannelsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">createBinding</a>(project_id, { ...params }) -> Sikaru.Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.createBinding("project_id", {
    agent_id: "agent_id",
    transport: "http"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateBinding` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">deleteHttpBinding</a>(project_id, binding_id) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.deleteHttpBinding("project_id", "binding_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">status</a>(project_id, binding_id, { ...params }) -> Sikaru.Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.status("project_id", "binding_id", {
    status: "active"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.BindingStatus` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">configureHttpBinding</a>(project_id, binding_id, { ...params }) -> Sikaru.Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.configureHttpBinding("project_id", "binding_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.HttpConfiguration` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">issueHttpCredential</a>(project_id, binding_id) -> Sikaru.HttpCredential</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.issueHttpCredential("project_id", "binding_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">configurePersonalAccess</a>(project_id, binding_id, { ...params }) -> Sikaru.PersonalAccessConfiguration</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.configurePersonalAccess("project_id", "binding_id", {
    enabled: true
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PersonalAccess` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">recentReceipts</a>(project_id, binding_id) -> Sikaru.ReceiptSummary[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.recentReceipts("project_id", "binding_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">getMemberSlackLink</a>(project_id, binding_id, verification_id) -> Sikaru.PersonalSlackStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.getMemberSlackLink("project_id", "binding_id", "verification_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">startMemberSlackLink</a>(project_id, binding_id, { ...params }) -> Sikaru.PersonalSlackChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.startMemberSlackLink("project_id", "binding_id", {
    installation_id: "installation_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SlackLink` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">unlinkMemberSlackIdentity</a>(project_id, binding_id, installation_id) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.unlinkMemberSlackIdentity("project_id", "binding_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**binding_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">saveCreationResume</a>(project_id, { ...params }) -> Sikaru.CreationResumeId</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.saveCreationResume("project_id", {
    payload: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreationResume` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">getCreationResume</a>(project_id, resume_id) -> Sikaru.CreationResume</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.getCreationResume("project_id", "resume_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**resume_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">listIdentityApps</a>(project_id) -> Sikaru.ChannelIdentityApp[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.listIdentityApps("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">createIdentityApp</a>(project_id, { ...params }) -> Sikaru.ChannelIdentityAppId</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.createIdentityApp("project_id", {
    audience: "audience",
    issuer: "issuer",
    public_jwk: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.IdentityApp` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">revokeIdentityApp</a>(project_id, app_id) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.revokeIdentityApp("project_id", "app_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">updateIdentityAppUrls</a>(project_id, app_id, { ...params }) -> Sikaru.ChannelIdentityAppUrLs</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.updateIdentityAppUrls("project_id", "app_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ApplicationUrLs` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">revokeSubject</a>(project_id, app_id, { ...params }) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.revokeSubject("project_id", "app_id", {
    subject: "subject",
    tenant_id: "tenant_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.Subject` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">createPersonalSlackBinding</a>(project_id, { ...params }) -> Sikaru.Binding</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.createPersonalSlackBinding("project_id", {
    agent_id: "agent_id",
    identity_app_id: "identity_app_id",
    installation_id: "installation_id",
    verification_id: "verification_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CustomerDm` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">delivery</a>(project_id, run_id) -> Sikaru.Delivery | null</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.delivery("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">resendDelivery</a>(project_id, run_id, { ...params }) -> Sikaru.Delivery</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.resendDelivery("project_id", "run_id", {
    acknowledge_possible_duplicate: true
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ResendDelivery` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">dmStatus</a>(project_id, verification_id) -> Sikaru.DmStatus</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.dmStatus("project_id", "verification_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**verification_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">installations</a>(project_id) -> Sikaru.Installation[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.installations("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">disconnect</a>(project_id, installation_id) -> Sikaru.Installation</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.disconnect("project_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">startDm</a>(project_id, installation_id) -> Sikaru.DmChallenge</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.startDm("project_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">rooms</a>(project_id, installation_id) -> Sikaru.Room[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.rooms("project_id", "installation_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**installation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">complete</a>(project_id, { ...params }) -> Sikaru.OAuthResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.complete("project_id", {
    state: "state"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CompleteOAuth` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client/Client.ts">start</a>(project_id, { ...params }) -> Sikaru.Authorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.channels.start("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartOAuth` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChannelsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeAttachments
<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">get</a>(project_id, attachment_id) -> Sikaru.AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.get("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">abandon</a>(project_id, attachment_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.abandon("project_id", "attachment_id", {
    evidence: "evidence"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.AbandonInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">cancel</a>(project_id, attachment_id) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.cancel("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">claim</a>(project_id, attachment_id, { ...params }) -> Sikaru.ClaimView</code></summary>
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

```typescript
await client.computeAttachments.claim("project_id", "attachment_id", {
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ClaimInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">cleanup</a>(project_id, attachment_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.cleanup("project_id", "attachment_id", {
    children_terminated: true,
    evidence: "evidence"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CleanupInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">connect</a>(project_id, attachment_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.connect("project_id", "attachment_id", {
    capabilities: ["compute.execute"],
    executor_instance_id: "executor_instance_id",
    journal_id: "journal_id",
    protocol_version: "sikaru-compute-v1",
    workspace_provenance: {
        identity: "identity",
        kind: "existing_directory"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReadyInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">issueCredential</a>(project_id, attachment_id, { ...params }) -> Sikaru.CredentialIssued</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.issueCredential("project_id", "attachment_id", {
    owner_epoch: 1,
    owner_id: "owner_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ExecutorCredentialInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">heartbeat</a>(project_id, attachment_id) -> Sikaru.AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.heartbeat("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">ready</a>(project_id, attachment_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.ready("project_id", "attachment_id", {
    capabilities: ["compute.execute"],
    executor_instance_id: "executor_instance_id",
    journal_id: "journal_id",
    protocol_version: "sikaru-compute-v1",
    workspace_provenance: {
        identity: "identity",
        kind: "existing_directory"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReadyInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">reconcile</a>(project_id, attachment_id, { ...params }) -> Sikaru.ReconcileView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.reconcile("project_id", "attachment_id", {
    executor_instance_id: "executor_instance_id",
    journal_id: "journal_id",
    workspace_provenance: {
        identity: "identity",
        kind: "existing_directory"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReconcileInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">status</a>(project_id, attachment_id) -> Sikaru.AttachmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeAttachments.status("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">stop</a>(project_id, attachment_id) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.stop("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">teardown</a>(project_id, attachment_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.teardown("project_id", "attachment_id", {
    children_terminated: true,
    evidence: "evidence",
    owner_epoch: 1,
    owner_id: "owner_id",
    workspace_generation: "workspace_generation"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.TeardownInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeAttachments.<a href="/src/api/resources/computeAttachments/client/Client.ts">create</a>(project_id, session_id, { ...params }) -> Sikaru.AttachmentView</code></summary>
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

```typescript
await client.computeAttachments.create("project_id", "session_id", {
    environment_id: "environment_id",
    idempotency_key: "idempotency_key",
    workspace_provenance: {
        identity: "identity",
        kind: "existing_directory"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.AttachmentInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeAttachmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeOperations
<details><summary><code>client.computeOperations.<a href="/src/api/resources/computeOperations/client/Client.ts">submitReceipt</a>(project_id, attachment_id, { ...params }) -> Sikaru.ReceiptView</code></summary>
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

```typescript
await client.computeOperations.submitReceipt("project_id", "attachment_id", {
    idempotency_key: "idempotency_key",
    payload: {
        "key": "value"
    },
    request_digest: "request_digest",
    run_id: "run_id",
    status: "completed",
    tool_call_id: "tool_call_id",
    tool_provider_id: "tool_provider_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReceiptInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeOperationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeOperations.<a href="/src/api/resources/computeOperations/client/Client.ts">poll</a>(project_id, attachment_id, { ...params }) -> Sikaru.WorkPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeOperations.poll("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PollComputeOperationsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeOperationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeWorkspaces
<details><summary><code>client.computeWorkspaces.<a href="/src/api/resources/computeWorkspaces/client/Client.ts">record</a>(project_id, attachment_id, { ...params }) -> Sikaru.WorkspaceCheckpointView</code></summary>
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

```typescript
await client.computeWorkspaces.record("project_id", "attachment_id", {
    manifest_digest: "manifest_digest",
    trigger: "turn"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.WorkspaceCheckpointInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeWorkspacesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeWorkspaces.<a href="/src/api/resources/computeWorkspaces/client/Client.ts">policy</a>(project_id, attachment_id, { ...params }) -> Sikaru.WorkspacePolicyView</code></summary>
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

```typescript
await client.computeWorkspaces.policy("project_id", "attachment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PolicyComputeWorkspacesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeWorkspacesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeCredentials
<details><summary><code>client.computeCredentials.<a href="/src/api/resources/computeCredentials/client/Client.ts">renew</a>(project_id) -> Sikaru.CredentialRenewed</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeCredentials.renew("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeCredentialsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeCredentials.<a href="/src/api/resources/computeCredentials/client/Client.ts">revoke</a>(project_id, credential_id) -> Sikaru.CredentialRevoked</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeCredentials.revoke("project_id", "credential_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**credential_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeCredentialsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ComputeEnvironments
<details><summary><code>client.computeEnvironments.<a href="/src/api/resources/computeEnvironments/client/Client.ts">create</a>(project_id, { ...params }) -> Sikaru.EnvironmentView</code></summary>
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

```typescript
await client.computeEnvironments.create("project_id", {
    environment_slug: "environment_slug",
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.EnvironmentInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeEnvironmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeEnvironments.<a href="/src/api/resources/computeEnvironments/client/Client.ts">get</a>(project_id, environment_id) -> Sikaru.EnvironmentView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.computeEnvironments.get("project_id", "environment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeEnvironmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.computeEnvironments.<a href="/src/api/resources/computeEnvironments/client/Client.ts">disable</a>(project_id, environment_id) -> Sikaru.EnvironmentView</code></summary>
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

```typescript
await client.computeEnvironments.disable("project_id", "environment_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**environment_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ComputeEnvironmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Connections
<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">listConnections</a>(project_id) -> Sikaru.Connection[]</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.listConnections("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">createConnection</a>(project_id, { ...params }) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.createConnection("project_id", {
    config: {},
    display_name: "display_name",
    kind: "mcp"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateConnection` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">listApps</a>(project_id, { ...params }) -> Sikaru.ConnectionApps</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.listApps("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListAppsConnectionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">getConnection</a>(project_id, connection_id) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.getConnection("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">updateConnection</a>(project_id, connection_id, { ...params }) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.updateConnection("project_id", "connection_id", {
    expected_version: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateConnection` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">authorize</a>(project_id, connection_id) -> Sikaru.ConnectionAuthorization</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.authorize("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">complete</a>(project_id, connection_id, { ...params }) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.complete("project_id", "connection_id", {
    state: "state"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CompleteAuthorization` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">credentials</a>(project_id, connection_id, { ...params }) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.credentials("project_id", "connection_id", {
    credentials: {}
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ReplaceCredentials` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">disable</a>(project_id, connection_id) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.disable("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">disconnect</a>(project_id, connection_id) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.disconnect("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">discover</a>(project_id, connection_id) -> Sikaru.Connection</code></summary>
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

```typescript
await client.connections.discover("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">enable</a>(project_id, connection_id) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.enable("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">events</a>(project_id, connection_id, { ...params }) -> Sikaru.ConnectionEventPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.events("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.EventsConnectionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">grant</a>(project_id, connection_id, { ...params }) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.grant("project_id", "connection_id", {
    agent_id: "agent_id",
    tools: ["tools"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.GrantConnection` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">revokeGrant</a>(project_id, connection_id, grant_id) -> Sikaru.Connection</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.revokeGrant("project_id", "connection_id", "grant_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**grant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">revoke</a>(project_id, connection_id) -> Sikaru.ConnectionRevocation</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.revoke("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connections.<a href="/src/api/resources/connections/client/Client.ts">usage</a>(project_id, connection_id) -> Sikaru.ConnectionUsage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.connections.usage("project_id", "connection_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**connection_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ContextRegistry
<details><summary><code>client.contextRegistry.<a href="/src/api/resources/contextRegistry/client/Client.ts">createContextRegistryChange</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.contextRegistry.createContextRegistryChange("project_id", {
    name: "name",
    repoId: "repoId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateContextRegistryChangeRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ContextRegistryClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Conversations
<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client/Client.ts">listMessages</a>(project_id, conversation_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.conversations.listMessages("project_id", "conversation_id", {
    account_id: "account_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListMessagesConversationsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConversationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client/Client.ts">recordMessage</a>(project_id, conversation_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.conversations.recordMessage("project_id", "conversation_id", {
    account_id: "account_id",
    content: "content",
    deliveredAt: "2024-01-15T09:30:00Z",
    messageId: "messageId",
    position: 1,
    role: "user"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DeliveredMessage` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ConversationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CreditTopUps
<details><summary><code>client.creditTopUps.<a href="/src/api/resources/creditTopUps/client/Client.ts">create</a>(project_id, { ...params }) -> Sikaru.CreditTopUp</code></summary>
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

```typescript
await client.creditTopUps.create("project_id", {
    amountUsd: "amountUsd"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.TopUpInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CreditTopUpsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.creditTopUps.<a href="/src/api/resources/creditTopUps/client/Client.ts">get</a>(project_id, top_up_id) -> Sikaru.CreditTopUp</code></summary>
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

```typescript
await client.creditTopUps.get("project_id", "top_up_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**top_up_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CreditTopUpsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Datasets
<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">listDatasets</a>(project_id, { ...params }) -> Sikaru.DatasetList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.listDatasets("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListDatasetsDatasetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">createDataset</a>(project_id, { ...params }) -> Sikaru.DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.createDataset("project_id", {
    name: "name"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateDataset` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">captureIntoNewDataset</a>(project_id, { ...params }) -> Sikaru.BatchResult</code></summary>
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

```typescript
await client.datasets.captureIntoNewDataset("project_id", {
    dataset: {
        name: "name"
    },
    idempotency_key: "idempotency_key",
    items: [{
            run_id: "run_id"
        }]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CaptureIntoNewRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">getDataset</a>(project_id, dataset_id) -> Sikaru.DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.getDataset("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">deleteDataset</a>(project_id, dataset_id) -> void</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.deleteDataset("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">updateDataset</a>(project_id, dataset_id, { ...params }) -> Sikaru.DatasetResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.updateDataset("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateDataset` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">captureIntoDataset</a>(project_id, dataset_id, { ...params }) -> Sikaru.BatchResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.captureIntoDataset("project_id", "dataset_id", {
    idempotency_key: "idempotency_key",
    items: [{
            run_id: "run_id"
        }]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CaptureRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">startDatasetChecks</a>(project_id, dataset_id, { ...params }) -> Sikaru.DatasetChecksStarted</code></summary>
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

```typescript
await client.datasets.startDatasetChecks("project_id", "dataset_id", {
    agent_slug: "agent_slug",
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartDatasetChecks` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">previewDatasetChecks</a>(project_id, dataset_id, { ...params }) -> Sikaru.DatasetChecksPreview</code></summary>
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

```typescript
await client.datasets.previewDatasetChecks("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PreviewDatasetChecksDatasetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">listDatasetCheckResults</a>(project_id, dataset_id, { ...params }) -> Sikaru.DatasetCheckResults</code></summary>
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

```typescript
await client.datasets.listDatasetCheckResults("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListDatasetCheckResultsDatasetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">listExamples</a>(project_id, dataset_id, { ...params }) -> Sikaru.DatasetExamplePage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.listExamples("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListExamplesDatasetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">deleteExample</a>(project_id, dataset_id, example_id) -> Sikaru.DatasetVersionResponse</code></summary>
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

```typescript
await client.datasets.deleteExample("project_id", "dataset_id", "example_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**example_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">updateExample</a>(project_id, dataset_id, example_id, { ...params }) -> Sikaru.DatasetExampleResponse</code></summary>
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

```typescript
await client.datasets.updateExample("project_id", "dataset_id", "example_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**example_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateExample` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">exportDataset</a>(project_id, dataset_id, { ...params }) -> core.BinaryResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.exportDataset("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ExportDatasetDatasetsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">uploadExamples</a>(project_id, dataset_id, { ...params }) -> Sikaru.BatchResult</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.uploadExamples("project_id", "dataset_id", {
    content: "content",
    format: "csv",
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UploadRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client/Client.ts">listVersions</a>(project_id, dataset_id) -> Sikaru.DatasetVersionList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.datasets.listVersions("project_id", "dataset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**dataset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DatasetsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Deployments
<details><summary><code>client.deployments.<a href="/src/api/resources/deployments/client/Client.ts">listConsoleDeployments</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.deployments.listConsoleDeployments("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DeploymentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Environments
<details><summary><code>client.environments.<a href="/src/api/resources/environments/client/Client.ts">listManagedEnvironments</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.environments.listManagedEnvironments("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EnvironmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client/Client.ts">createManagedEnvironment</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.environments.createManagedEnvironment("project_id", {
    environmentSlug: "environmentSlug",
    providerType: "providerType",
    runtimeProvider: "runtimeProvider"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateManagedEnvironmentRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EnvironmentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationComparisons
<details><summary><code>client.evaluationComparisons.<a href="/src/api/resources/evaluationComparisons/client/Client.ts">listComparisons</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationComparisons.listComparisons("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListComparisonsEvaluationComparisonsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationComparisonsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationComparisons.<a href="/src/api/resources/evaluationComparisons/client/Client.ts">createComparison</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationComparisons.createComparison("project_id", {
    cases: [{
            baseline: {
                accountId: "accountId",
                conversationId: "conversationId",
                messageId: "messageId"
            },
            candidate: {
                accountId: "accountId",
                conversationId: "conversationId",
                messageId: "messageId"
            },
            caseId: "caseId"
        }],
    evaluator: "evaluator",
    id: "id",
    revision: "revision",
    rubric: "rubric"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ComparisonInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationComparisonsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationComparisons.<a href="/src/api/resources/evaluationComparisons/client/Client.ts">getComparison</a>(project_id, comparison_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationComparisons.getComparison("project_id", "comparison_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**comparison_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationComparisonsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationComparisons.<a href="/src/api/resources/evaluationComparisons/client/Client.ts">cancelComparison</a>(project_id, comparison_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationComparisons.cancelComparison("project_id", "comparison_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**comparison_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationComparisonsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationCriteria
<details><summary><code>client.evaluationCriteria.<a href="/src/api/resources/evaluationCriteria/client/Client.ts">listCriteria</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationCriteria.listCriteria("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListCriteriaEvaluationCriteriaRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationCriteriaClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationJobs
<details><summary><code>client.evaluationJobs.<a href="/src/api/resources/evaluationJobs/client/Client.ts">listJobs</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationJobs.listJobs("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListJobsEvaluationJobsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationJobsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationJobs.<a href="/src/api/resources/evaluationJobs/client/Client.ts">createJob</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationJobs.createJob("project_id", {
    evaluator: "evaluator",
    requestId: "requestId",
    revision: "revision",
    rubric: "rubric",
    targets: [{
            target: {
                accountId: "accountId",
                kind: "message"
            },
            traceId: "traceId"
        }]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.JobInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationJobsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationJobs.<a href="/src/api/resources/evaluationJobs/client/Client.ts">getJob</a>(project_id, job_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationJobs.getJob("project_id", "job_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationJobsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationJobs.<a href="/src/api/resources/evaluationJobs/client/Client.ts">cancelJob</a>(project_id, job_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationJobs.cancelJob("project_id", "job_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationJobsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluationResults
<details><summary><code>client.evaluationResults.<a href="/src/api/resources/evaluationResults/client/Client.ts">listResults</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationResults.listResults("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListResultsEvaluationResultsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationResultsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluationResults.<a href="/src/api/resources/evaluationResults/client/Client.ts">recordResult</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluationResults.recordResult("project_id", {
    environment: "production",
    evaluator: "evaluator",
    id: "id",
    publicReason: "publicReason",
    revision: "revision",
    source: "human",
    traceId: "traceId",
    verdict: "pass"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.EvaluationResult` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluationResultsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EvaluatorRuns
<details><summary><code>client.evaluatorRuns.<a href="/src/api/resources/evaluatorRuns/client/Client.ts">createEvaluatorRun</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.evaluatorRuns.createEvaluatorRun("project_id", {
    traceIds: ["traceIds"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateEvaluatorRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EvaluatorRunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ExecutionObjectives
<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">listObjectives</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.listObjectives("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListObjectivesExecutionObjectivesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">create</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.create("project_id", {
    criterion: "criterion",
    idempotency_key: "idempotency_key",
    objective: "objective",
    run_id: "run_id",
    session_id: "session_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ObjectiveInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">get</a>(project_id, objective_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.get("project_id", "objective_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">cancel</a>(project_id, objective_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.cancel("project_id", "objective_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">pause</a>(project_id, objective_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.pause("project_id", "objective_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionObjectives.<a href="/src/api/resources/executionObjectives/client/Client.ts">resume</a>(project_id, objective_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionObjectives.resume("project_id", "objective_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**objective_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ResumeInput | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionObjectivesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ExecutionSessions
<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">list</a>(project_id, { ...params }) -> Sikaru.ExecutionSessionPage</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.list("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListExecutionSessionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">get</a>(project_id, session_id) -> Sikaru.ExecutionSessionResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.get("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">branch</a>(project_id, session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.branch("project_id", "session_id", {
    idempotency_key: "idempotency_key",
    source_run_id: "source_run_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.BranchInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">listFiles</a>(project_id, session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.listFiles("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">deleteFile</a>(project_id, session_id, file_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.deleteFile("project_id", "session_id", "file_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">downloadFile</a>(project_id, session_id, file_id) -> core.BinaryResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.downloadFile("project_id", "session_id", "file_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">listSessionInputs</a>(project_id, session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.listSessionInputs("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">spend</a>(project_id, session_id) -> Sikaru.SessionSpend</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.spend("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">appendTurn</a>(project_id, session_id, { ...params }) -> Sikaru.ExecutionTurnResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.appendTurn("project_id", "session_id", {
    idempotency_key: "idempotency_key",
    input: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.TurnInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">create</a>(project_id, harness_id, { ...params }) -> Sikaru.ExecutionSessionResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executionSessions.create("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SessionInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.executionSessions.<a href="/src/api/resources/executionSessions/client/Client.ts">listEligibleReleases</a>(project_id, harness_id, { ...params }) -> Sikaru.EligibleReleases</code></summary>
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

```typescript
await client.executionSessions.listEligibleReleases("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListEligibleReleasesExecutionSessionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Specialists
<details><summary><code>client.specialists.<a href="/src/api/resources/specialists/client/Client.ts">list</a>(project_id, session_id) -> Sikaru.SpecialistThreads</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.specialists.list("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SpecialistsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="/src/api/resources/specialists/client/Client.ts">get</a>(project_id, session_id, thread_id) -> Sikaru.SpecialistThread</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.specialists.get("project_id", "session_id", "thread_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SpecialistsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="/src/api/resources/specialists/client/Client.ts">cancel</a>(project_id, session_id, thread_id, { ...params }) -> Sikaru.SpecialistReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.specialists.cancel("project_id", "session_id", "thread_id", {
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SpecialistCancel` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SpecialistsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.specialists.<a href="/src/api/resources/specialists/client/Client.ts">message</a>(project_id, session_id, thread_id, { ...params }) -> Sikaru.SpecialistReceipt</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.specialists.message("project_id", "session_id", "thread_id", {
    idempotency_key: "idempotency_key",
    input: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SpecialistMessage` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SpecialistsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Executions
<details><summary><code>client.executions.<a href="/src/api/resources/executions/client/Client.ts">executionRuntimeLineage</a>(project_id, trace_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.executions.executionRuntimeLineage("project_id", "trace_id", {
    account_id: "account_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ExecutionRuntimeLineageExecutionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ExecutionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Feedback
<details><summary><code>client.feedback.<a href="/src/api/resources/feedback/client/Client.ts">createFeedback</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.feedback.createFeedback("project_id", {
    kind: "thumbs_up",
    target: "trace",
    targetId: "targetId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateFeedbackRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `FeedbackClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## GitCredentials
<details><summary><code>client.gitCredentials.<a href="/src/api/resources/gitCredentials/client/Client.ts">list</a>(project_id) -> Sikaru.GitCredential[]</code></summary>
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

```typescript
await client.gitCredentials.list("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `GitCredentialsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.gitCredentials.<a href="/src/api/resources/gitCredentials/client/Client.ts">create</a>(project_id, { ...params }) -> Sikaru.GitCredential</code></summary>
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

```typescript
await client.gitCredentials.create("project_id", {
    host: "host",
    token: "token"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateGitCredential` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `GitCredentialsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.gitCredentials.<a href="/src/api/resources/gitCredentials/client/Client.ts">grant</a>(project_id, credential_id, { ...params }) -> Sikaru.GitCredential</code></summary>
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

```typescript
await client.gitCredentials.grant("project_id", "credential_id", {
    agentId: "agentId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**credential_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.GrantGitCredential` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `GitCredentialsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Harnesses
<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">getInvoiceBudget</a>(project_id, harness_id) -> Sikaru.InvoiceBudget</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.getInvoiceBudget("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">improvementOptions</a>(project_id, harness_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.improvementOptions("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">listImprovements</a>(project_id, harness_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.listImprovements("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListImprovementsHarnessesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">startImprovement</a>(project_id, harness_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.startImprovement("project_id", "harness_id", {
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ImprovementInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">getImprovement</a>(project_id, harness_id, job_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.getImprovement("project_id", "harness_id", "job_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">resumeImprovement</a>(project_id, harness_id, job_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.harnesses.resumeImprovement("project_id", "harness_id", "job_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**job_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ResumeImprovementInput | null` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.harnesses.<a href="/src/api/resources/harnesses/client/Client.ts">trainModelStub</a>(project_id, harness_id) -> Record&lt;string, unknown&gt;</code></summary>
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

```typescript
await client.harnesses.trainModelStub("project_id", "harness_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `HarnessesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Runs
<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">start</a>(project_id, harness_id, { ...params }) -> Sikaru.ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.start("project_id", "harness_id", {
    input: {
        "key": "value"
    },
    policy: {
        "key": "value"
    },
    product_context: {
        "key": "value"
    },
    tenant_id: "tenant_id",
    user_id: "user_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**harness_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartHarnessRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">record</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
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

```typescript
await client.runs.record("project_id", {
    input: "input",
    output: "output"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RecordRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">resumeAll</a>(project_id, { ...params }) -> Sikaru.ResumeResults</code></summary>
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

```typescript
await client.runs.resumeAll("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ResumeFilter` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">resumeCount</a>(project_id) -> Sikaru.ResumeCounts</code></summary>
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

```typescript
await client.runs.resumeCount("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">get</a>(project_id, run_id) -> Sikaru.ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.get("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">pendingActions</a>(project_id, run_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.pendingActions("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">cancel</a>(project_id, run_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.cancel("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">events</a>(project_id, run_id, { ...params }) -> Sikaru.RunEvents</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.events("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.EventsRunsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">streamEvents</a>(project_id, run_id, { ...params }) -> core.Stream&lt;Sikaru.RunEvent&gt;</code></summary>
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

```typescript
const response = await client.runs.streamEvents("project_id", "run_id");
for await (const item of response) {
    console.log(item);
}

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StreamEventsRunsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">recover</a>(project_id, run_id, { ...params }) -> Sikaru.ManagedRun</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.recover("project_id", "run_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RecoverRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">resume</a>(project_id, run_id) -> Sikaru.ResumeResult</code></summary>
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

```typescript
await client.runs.resume("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">retry</a>(project_id, run_id) -> Sikaru.ExecutionTurnResponse</code></summary>
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

```typescript
await client.runs.retry("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">decideApproval</a>(project_id, run_id, tool_call_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.decideApproval("project_id", "run_id", "tool_call_id", {
    decision: "approved",
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tool_call_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ApprovalInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">submitToolResult</a>(project_id, run_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.submitToolResult("project_id", "run_id", {
    capability_name: "capability_name",
    idempotency_key: "idempotency_key",
    payload: {
        "key": "value"
    },
    status: "completed",
    tool_call_id: "tool_call_id",
    tool_provider_id: "tool_provider_id"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.SubmitToolResultRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">getTrajectory</a>(project_id, run_id) -> Record&lt;string, unknown&gt;</code></summary>
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

```typescript
await client.runs.getTrajectory("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">getTranscript</a>(project_id, run_id) -> Sikaru.RunTranscript</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runs.getTranscript("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runs.<a href="/src/api/resources/runs/client/Client.ts">streamTranscriptEvents</a>(project_id, run_id, { ...params }) -> core.Stream&lt;Sikaru.TranscriptStreamEvent&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
const response = await client.runs.streamTranscriptEvents("project_id", "run_id");
for await (const item of response) {
    console.log(item);
}

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StreamTranscriptEventsRunsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ImportSessions
<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">listImportSessions</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.listImportSessions("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">createImportSession</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.createImportSession("project_id", {
    displayName: "displayName",
    mode: "workflow"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateImportSessionRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">getImportSession</a>(project_id, import_session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.getImportSession("project_id", "import_session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">createCompatibilityProfile</a>(project_id, import_session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.createCompatibilityProfile("project_id", "import_session_id", {
    displayName: "displayName",
    harnessId: "harnessId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateCompatibilityProfileRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">listImportSessionDiffs</a>(project_id, import_session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.listImportSessionDiffs("project_id", "import_session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">getParityReport</a>(project_id, import_session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.getParityReport("project_id", "import_session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">promoteImportSession</a>(project_id, import_session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.promoteImportSession("project_id", "import_session_id", {
    activeHarnessVersionId: "activeHarnessVersionId",
    agentSlug: "agentSlug",
    compatibilityProfileId: "compatibilityProfileId",
    displayName: "displayName",
    harnessId: "harnessId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PromoteImportSessionRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">createReplayRun</a>(project_id, import_session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.createReplayRun("project_id", "import_session_id", {
    compatibilityProfileId: "compatibilityProfileId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateParityRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">listSourceArtifacts</a>(project_id, import_session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.listSourceArtifacts("project_id", "import_session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">createSourceArtifact</a>(project_id, import_session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.createSourceArtifact("project_id", "import_session_id", {
    contentDigest: "contentDigest",
    exportPolicy: "customer_source_exportable",
    sourceKind: "workflow_json",
    storageRef: "storageRef"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateSourceArtifactRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.importSessions.<a href="/src/api/resources/importSessions/client/Client.ts">createStagingRun</a>(project_id, import_session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.importSessions.createStagingRun("project_id", "import_session_id", {
    compatibilityProfileId: "compatibilityProfileId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**import_session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateParityRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ImportSessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## IssueClusters
<details><summary><code>client.issueClusters.<a href="/src/api/resources/issueClusters/client/Client.ts">listIssueClusters</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.issueClusters.listIssueClusters("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListIssueClustersIssueClustersRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `IssueClustersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issueClusters.<a href="/src/api/resources/issueClusters/client/Client.ts">upsertIssueCluster</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.issueClusters.upsertIssueCluster("project_id", {
    label: "label",
    traceIds: ["traceIds"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpsertIssueClusterRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `IssueClustersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issueClusters.<a href="/src/api/resources/issueClusters/client/Client.ts">getIssueCluster</a>(project_id, cluster_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.issueClusters.getIssueCluster("project_id", "cluster_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `IssueClustersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issueClusters.<a href="/src/api/resources/issueClusters/client/Client.ts">updateIssueClusterStatus</a>(project_id, cluster_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.issueClusters.updateIssueClusterStatus("project_id", "cluster_id", {
    status: "status"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateIssueClusterStatusRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `IssueClustersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.issueClusters.<a href="/src/api/resources/issueClusters/client/Client.ts">proposeIssueClusterFix</a>(project_id, cluster_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.issueClusters.proposeIssueClusterFix("project_id", "cluster_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `IssueClustersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## JudgeAlignment
<details><summary><code>client.judgeAlignment.<a href="/src/api/resources/judgeAlignment/client/Client.ts">getJudgeAlignment</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.judgeAlignment.getJudgeAlignment("project_id", {
    evaluator: "evaluator",
    revision: "revision"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.GetJudgeAlignmentJudgeAlignmentRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `JudgeAlignmentClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ManagedAgents
<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">listManagedAgents</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.managedAgents.listManagedAgents("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">createManagedAgent</a>(project_id, { ...params }) -> Sikaru.CreatedManagedAgent</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.managedAgents.createManagedAgent("project_id", {
    agentSlug: "agentSlug"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateManagedAgentRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">get</a>(project_id, agent_slug) -> Sikaru.ManagedAgentRead</code></summary>
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

```typescript
await client.managedAgents.get("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">deleteManagedAgent</a>(project_id, agent_slug) -> void</code></summary>
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

```typescript
await client.managedAgents.deleteManagedAgent("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">update</a>(project_id, agent_slug, { ...params }) -> Sikaru.RenamedManagedAgent</code></summary>
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

```typescript
await client.managedAgents.update("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateAgent` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">createDefinitionRevision</a>(project_id, agent_slug, { ...params }) -> Sikaru.DefinitionRevisionResult</code></summary>
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

```typescript
await client.managedAgents.createDefinitionRevision("project_id", "agent_slug", {
    contentDigest: "contentDigest",
    definition: {
        schema: "sikaru.agent.contract.v1"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DefinitionRevisionRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.managedAgents.<a href="/src/api/resources/managedAgents/client/Client.ts">getDefinitionRevision</a>(project_id, agent_slug, changeset_id) -> Sikaru.DefinitionRevisionView</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.managedAgents.getDefinitionRevision("project_id", "agent_slug", "changeset_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**changeset_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ManagedAgentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Checks
<details><summary><code>client.checks.<a href="/src/api/resources/checks/client/Client.ts">list</a>(project_id, agent_slug) -> Sikaru.CheckList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.checks.list("project_id", "agent_slug");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChecksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="/src/api/resources/checks/client/Client.ts">create</a>(project_id, agent_slug, { ...params }) -> Sikaru.CheckResponse</code></summary>
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

```typescript
await client.checks.create("project_id", "agent_slug", {
    environment: {
        kind: "managed"
    },
    idempotency_key: "idempotency_key",
    name: "name",
    task: {
        files: {
            "key": "value"
        }
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateCheck` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChecksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="/src/api/resources/checks/client/Client.ts">listResults</a>(project_id, agent_slug, check_id) -> Sikaru.CheckResultList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.checks.listResults("project_id", "agent_slug", "check_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**check_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChecksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checks.<a href="/src/api/resources/checks/client/Client.ts">run</a>(project_id, agent_slug, check_id, { ...params }) -> Sikaru.CheckResultResponse</code></summary>
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

```typescript
await client.checks.run("project_id", "agent_slug", "check_id", {
    idempotency_key: "idempotency_key"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**check_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RunCheck` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ChecksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EndUsers
<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">get</a>(project_id, agent_slug, tenant_id, user_id) -> Sikaru.EndUser</code></summary>
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

```typescript
await client.endUsers.get("project_id", "agent_slug", "tenant_id", "user_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">upsert</a>(project_id, agent_slug, tenant_id, user_id, { ...params }) -> Sikaru.EndUser</code></summary>
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

```typescript
await client.endUsers.upsert("project_id", "agent_slug", "tenant_id", "user_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpsertEndUser` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">delete</a>(project_id, agent_slug, tenant_id, user_id) -> Sikaru.EndUserDeletion</code></summary>
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

```typescript
await client.endUsers.delete("project_id", "agent_slug", "tenant_id", "user_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">updateFact</a>(project_id, agent_slug, tenant_id, user_id, { ...params }) -> Sikaru.EndUserFile</code></summary>
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

```typescript
await client.endUsers.updateFact("project_id", "agent_slug", "tenant_id", "user_id", {
    content: "content",
    path: "path"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.UpdateEndUserFact` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">deleteFact</a>(project_id, agent_slug, tenant_id, user_id, { ...params }) -> void</code></summary>
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

```typescript
await client.endUsers.deleteFact("project_id", "agent_slug", "tenant_id", "user_id", {
    path: "path"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.DeleteFactEndUsersRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.endUsers.<a href="/src/api/resources/endUsers/client/Client.ts">correctPersonaSummary</a>(project_id, agent_slug, tenant_id, user_id, { ...params }) -> Sikaru.RecordedCorrection</code></summary>
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

```typescript
await client.endUsers.correctPersonaSummary("project_id", "agent_slug", "tenant_id", "user_id", {
    text: "text"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**agent_slug:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PersonaSummaryCorrection` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `EndUsersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## MemoryRegistry
<details><summary><code>client.memoryRegistry.<a href="/src/api/resources/memoryRegistry/client/Client.ts">createMemoryRegistryChange</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.memoryRegistry.createMemoryRegistryChange("project_id", {
    memoryId: "memoryId",
    memoryType: "memoryType",
    name: "name",
    scope: "scope"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateMemoryRegistryChangeRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `MemoryRegistryClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ModelGateway
<details><summary><code>client.modelGateway.<a href="/src/api/resources/modelGateway/client/Client.ts">captureModelGatewayChatCompletion</a>(project_id, provider, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.modelGateway.captureModelGatewayChatCompletion("project_id", "provider", {
    request: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ModelGatewayCaptureRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ModelGatewayClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ModelSettings
<details><summary><code>client.modelSettings.<a href="/src/api/resources/modelSettings/client/Client.ts">getModelSettings</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.modelSettings.getModelSettings("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ModelSettingsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.modelSettings.<a href="/src/api/resources/modelSettings/client/Client.ts">updateModelSettings</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.modelSettings.updateModelSettings("project_id", {
    selectedModelId: "selectedModelId"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ModelSelection` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ModelSettingsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## OnlineEvaluations
<details><summary><code>client.onlineEvaluations.<a href="/src/api/resources/onlineEvaluations/client/Client.ts">listPolicies</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.onlineEvaluations.listPolicies("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListPoliciesOnlineEvaluationsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `OnlineEvaluationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.onlineEvaluations.<a href="/src/api/resources/onlineEvaluations/client/Client.ts">createPolicy</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.onlineEvaluations.createPolicy("project_id", {
    evaluator: "evaluator",
    id: "id",
    revision: "revision",
    rubric: "rubric",
    samplePercent: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PolicyInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `OnlineEvaluationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.onlineEvaluations.<a href="/src/api/resources/onlineEvaluations/client/Client.ts">previewPolicyEligibility</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.onlineEvaluations.previewPolicyEligibility("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PreviewPolicyEligibilityOnlineEvaluationsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `OnlineEvaluationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.onlineEvaluations.<a href="/src/api/resources/onlineEvaluations/client/Client.ts">updatePolicy</a>(project_id, policy_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.onlineEvaluations.updatePolicy("project_id", "policy_id", {
    enabled: true
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**policy_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PolicyState` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `OnlineEvaluationsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ReleaseWatches
<details><summary><code>client.releaseWatches.<a href="/src/api/resources/releaseWatches/client/Client.ts">createReleaseWatch</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.releaseWatches.createReleaseWatch("project_id", {
    traceIds: ["traceIds"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateReleaseWatchRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ReleaseWatchesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RetentionPolicies
<details><summary><code>client.retentionPolicies.<a href="/src/api/resources/retentionPolicies/client/Client.ts">createRetentionPolicyUpdate</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.retentionPolicies.createRetentionPolicyUpdate("project_id", {
    traceIds: ["traceIds"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateRetentionPolicyUpdateRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RetentionPoliciesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ReviewQueue
<details><summary><code>client.reviewQueue.<a href="/src/api/resources/reviewQueue/client/Client.ts">createReviewQueueItem</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.reviewQueue.createReviewQueueItem("project_id", {
    traceIds: ["traceIds"]
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateReviewQueueItemRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ReviewQueueClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunReferences
<details><summary><code>client.runReferences.<a href="/src/api/resources/runReferences/client/Client.ts">resolveRunReference</a>(project_id, reference, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runReferences.resolveRunReference("project_id", "reference");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ResolveRunReferenceRunReferencesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunReferencesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunSchedules
<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">listSchedules</a>(project_id, { ...params }) -> Sikaru.ScheduleList</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.listSchedules("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListSchedulesRunSchedulesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">createSchedule</a>(project_id, { ...params }) -> Sikaru.ScheduleResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.createSchedule("project_id", {
    input: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ScheduleInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">deleteSchedule</a>(project_id, schedule_id) -> Sikaru.ScheduleDeleted</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.deleteSchedule("project_id", "schedule_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">pauseSchedule</a>(project_id, schedule_id, { ...params }) -> Sikaru.SchedulePaused</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.pauseSchedule("project_id", "schedule_id", {
    paused: true
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PauseInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">scheduleNotices</a>(project_id, schedule_id) -> Sikaru.ScheduleNotices</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.scheduleNotices("project_id", "schedule_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runSchedules.<a href="/src/api/resources/runSchedules/client/Client.ts">scheduleHistory</a>(project_id, schedule_id, { ...params }) -> Sikaru.ScheduleHistory</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runSchedules.scheduleHistory("project_id", "schedule_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ScheduleHistoryRunSchedulesRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunSchedulesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RunWebhooks
<details><summary><code>client.runWebhooks.<a href="/src/api/resources/runWebhooks/client/Client.ts">listWebhooks</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runWebhooks.listWebhooks("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunWebhooksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runWebhooks.<a href="/src/api/resources/runWebhooks/client/Client.ts">createWebhook</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runWebhooks.createWebhook("project_id", {
    run_id: "run_id",
    url: "url"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.WebhookInput` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunWebhooksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.runWebhooks.<a href="/src/api/resources/runWebhooks/client/Client.ts">deleteWebhook</a>(project_id, webhook_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.runWebhooks.deleteWebhook("project_id", "webhook_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `RunWebhooksClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Sessions
<details><summary><code>client.sessions.<a href="/src/api/resources/sessions/client/Client.ts">getManagedSession</a>(project_id, session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.sessions.getManagedSession("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="/src/api/resources/sessions/client/Client.ts">listManagedSessionEvents</a>(project_id, session_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.sessions.listManagedSessionEvents("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ListManagedSessionEventsSessionsRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="/src/api/resources/sessions/client/Client.ts">listManagedSessionFiles</a>(project_id, session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.sessions.listManagedSessionFiles("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sessions.<a href="/src/api/resources/sessions/client/Client.ts">listManagedSessionPlan</a>(project_id, session_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.sessions.listManagedSessionPlan("project_id", "session_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `SessionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ToolProviders
<details><summary><code>client.toolProviders.<a href="/src/api/resources/toolProviders/client/Client.ts">registerToolProvider</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.toolProviders.registerToolProvider("project_id", {
    broker_endpoint_ref: "broker_endpoint_ref",
    capability_prefix: "capability_prefix",
    display_name: "display_name",
    provider_type: "provider_type",
    tool_catalog_ref: "tool_catalog_ref"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RegisterToolProviderRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ToolProvidersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.toolProviders.<a href="/src/api/resources/toolProviders/client/Client.ts">attachSourceToolSkill</a>(project_id, tool_provider_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.toolProviders.attachSourceToolSkill("project_id", "tool_provider_id", {
    capability_refs: ["capability_refs"],
    description: "description",
    source: {
        kind: "kind"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**tool_provider_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.AttachSourceToolSkillRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ToolProvidersClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceImportConnections
<details><summary><code>client.traceImportConnections.<a href="/src/api/resources/traceImportConnections/client/Client.ts">listTraceImportConnections</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImportConnections.listTraceImportConnections("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportConnectionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceImports
<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">listTraceImports</a>(project_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.listTraceImports("project_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">createTraceImport</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.createTraceImport("project_id", {
    connectionId: "connectionId",
    converterVersion: "converterVersion",
    dataset: "dataset",
    externalProjectId: "externalProjectId",
    mode: "historical",
    provider: "langsmith",
    scope: {},
    sourceInstance: "sourceInstance"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateTraceImportRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">planTraceImport</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.planTraceImport("project_id", {
    connectionId: "connectionId",
    converterVersion: "converterVersion",
    dataset: "dataset",
    externalProjectId: "externalProjectId",
    mode: "historical",
    provider: "langsmith",
    scope: {},
    sourceInstance: "sourceInstance"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.PlanTraceImportRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">getTraceImport</a>(project_id, trace_import_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.getTraceImport("project_id", "trace_import_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">cancelTraceImport</a>(project_id, trace_import_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.cancelTraceImport("project_id", "trace_import_id", {
    expectedVersion: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.VersionedTraceImportAction` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">getTraceImportReceipt</a>(project_id, trace_import_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.getTraceImportReceipt("project_id", "trace_import_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.traceImports.<a href="/src/api/resources/traceImports/client/Client.ts">retryTraceImport</a>(project_id, trace_import_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceImports.retryTraceImport("project_id", "trace_import_id", {
    expectedVersion: 1
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**trace_import_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.VersionedTraceImportAction` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceImportsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## WorkflowIntents
<details><summary><code>client.workflowIntents.<a href="/src/api/resources/workflowIntents/client/Client.ts">createProjectWorkflowIntent</a>(project_id, { ...params }) -> Record&lt;string, string&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflowIntents.createProjectWorkflowIntent("project_id", {
    description: "description"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateProductWorkflowIntentRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowIntentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflowIntents.<a href="/src/api/resources/workflowIntents/client/Client.ts">compileProjectWorkflowIntent</a>(project_id, intent_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflowIntents.compileProjectWorkflowIntent("project_id", "intent_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**intent_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowIntentsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## WorkflowRuns
<details><summary><code>client.workflowRuns.<a href="/src/api/resources/workflowRuns/client/Client.ts">projectWorkflowRunEvents</a>(project_id, run_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflowRuns.projectWorkflowRunEvents("project_id", "run_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowRunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflowRuns.<a href="/src/api/resources/workflowRuns/client/Client.ts">recoverProjectWorkflowRun</a>(project_id, run_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflowRuns.recoverProjectWorkflowRun("project_id", "run_id", {});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.RecoverRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowRunsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Workflows
<details><summary><code>client.workflows.<a href="/src/api/resources/workflows/client/Client.ts">importWorkflow</a>(project_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflows.importWorkflow("project_id", {
    payload: {
        "key": "value"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.ImportWorkflowRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="/src/api/resources/workflows/client/Client.ts">exportProductWorkflow</a>(project_id, workflow_id) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflows.exportProductWorkflow("project_id", "workflow_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="/src/api/resources/workflows/client/Client.ts">startProjectWorkflowRun</a>(project_id, workflow_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflows.startProjectWorkflowRun("project_id", "workflow_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.StartWorkflowRunRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.workflows.<a href="/src/api/resources/workflows/client/Client.ts">createProjectWorkflowVersion</a>(project_id, workflow_id, { ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.workflows.createProjectWorkflowVersion("project_id", "workflow_id");

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**workflow_id:** `string` 
    
</dd>
</dl>

<dl>
<dd>

**request:** `Sikaru.CreateWorkflowVersionRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `WorkflowsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Responses
<details><summary><code>client.responses.<a href="/src/api/resources/responses/client/Client.ts">createStream</a>({ ...params }) -> core.Stream&lt;Sikaru.ResponseStreamEvent&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
const response = await client.responses.createStream({
    model: "model"
});
for await (const item of response) {
    console.log(item);
}

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.ResponseCreateParamsStreaming` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ResponsesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.responses.<a href="/src/api/resources/responses/client/Client.ts">create</a>({ ...params }) -> Sikaru.ModelResponse</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.responses.create({
    model: "model"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.ResponseCreateParams` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `ResponsesClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Decisions
<details><summary><code>client.decisions.<a href="/src/api/resources/decisions/client/Client.ts">create</a>({ ...params }) -> Sikaru.Decision</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.decisions.create({
    questions: {
        "key": {
            "key": "value"
        }
    },
    state: "state"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.DecisionCreateParams` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `DecisionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TraceStreams
<details><summary><code>client.traceStreams.<a href="/src/api/resources/traceStreams/client/Client.ts">streamOpeninferenceSpans</a>({ ...params }) -> Record&lt;string, unknown&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.traceStreams.streamOpeninferenceSpans({
    format: "openinference.v1",
    metadata: {
        account_id: "account_id",
        project_id: "project_id",
        source: "source"
    }
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.TraceStreamRequest` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `TraceStreamsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Chat Completions
<details><summary><code>client.chat.completions.<a href="/src/api/resources/chat/resources/completions/client/Client.ts">createStream</a>({ ...params }) -> core.Stream&lt;Sikaru.ChatCompletionChunk&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
const response = await client.chat.completions.createStream({
    messages: [{
            "key": "value"
        }],
    model: "model"
});
for await (const item of response) {
    console.log(item);
}

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.chat.ChatCompletionCreateParamsStreaming` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CompletionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.completions.<a href="/src/api/resources/chat/resources/completions/client/Client.ts">create</a>({ ...params }) -> Sikaru.ChatCompletion</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```typescript
await client.chat.completions.create({
    messages: [{
            "key": "value"
        }],
    model: "model"
});

```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request:** `Sikaru.chat.ChatCompletionCreateParams` 
    
</dd>
</dl>

<dl>
<dd>

**requestOptions:** `CompletionsClient.RequestOptions` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

