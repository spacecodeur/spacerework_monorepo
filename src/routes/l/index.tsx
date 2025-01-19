import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

interface IndexProps {
    content: string
}

export default function IndexPage({
    data,
    isLoading,
}: TuonoProps<IndexProps>): JSX.Element {
    if (isLoading) {
        return <h1>Loading...</h1>
    }



    return (
        <>
            <div>{data?.content}</div>
        </>
    )
}