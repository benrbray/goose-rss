import { ParentProps } from "solid-js"

export namespace ExternalLink {
  export type Props = ParentProps<{
    href: string|undefined
  }>;
}

export const ExternalLink = (props: ExternalLink.Props) => {
  return <a class="external" href={props.href} title={props.href} target="_blank" children={props.children} />
}