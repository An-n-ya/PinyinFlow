import { Field, FieldDescription, FieldGroup, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
export default function QWenSetting() {
    return (
        <FieldGroup>
            <Field>
                <FieldLabel htmlFor="input-demo-api-key">API Key</FieldLabel>
                <Input id="input-demo-api-key" type="password" placeholder="sk-..." />
                <FieldDescription>您的API密钥将被加密并安全存储。</FieldDescription>
            </Field>
            <Field>
                <Label htmlFor="username-1">模型名称</Label>
                <Input id="model" name="model" defaultValue="qwen3-tts-flash-realtime" />
            </Field>
        </FieldGroup>
    );
}
