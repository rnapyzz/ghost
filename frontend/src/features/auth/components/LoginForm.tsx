import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {Button} from "@/components/ui/button.tsx";

export function LoginForm() {
   return (
       <Card>
           <CardHeader>
               <CardTitle>ログイン</CardTitle>
           </CardHeader>
           <CardContent>
               <p>ここにフォーム作る</p>
               <Button>Login</Button>
           </CardContent>
       </Card>
   )
}