/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface TransformOutput {
  code: string
  map?: string
}
export interface Output {
  data: string
}
export interface Config {
  path?: string
}
export function optimize(input: string, config?: Config | undefined | null): Output
