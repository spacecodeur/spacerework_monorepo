import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'
import PathMode from '../../../components/lesson/PathMode'
import ReadMode from '../../../components/lesson/ReadMode'
import EditMode from '../../../components/lesson/EditMode'

export default function IndexPage({
    data,
    // isLoading,
}: TuonoProps<string>): JSX.Element {
    return (
        <>
            {data.path.status === "path" && <PathMode path={data.path.value} />}
            {data.path.status === "lesson" && <ReadMode path={data.path.value} lesson={data.lesson} lesson_html={data.lesson_html}/>}
            {data.path.status === "edit" && <EditMode path={data.path.value} lesson={data.lesson} />}
        </>
    )
}