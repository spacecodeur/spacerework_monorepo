import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

export default function IndexPage({
    data
}: TuonoProps<string>): JSX.Element {
    
    return (
        <>
            <div>trainers</div>
            {data.trainers.map((trainer)=><h2>{trainer.pseudo}</h2>)}
        </>
    )
}