import type { JSX } from 'react'
import { useRouter } from 'tuono'

export default function IndexPage(): JSX.Element {
    const router = useRouter();
    console.log(router);

    return (
        <>
            <div>trainer num :id</div>
        </>
    )
}