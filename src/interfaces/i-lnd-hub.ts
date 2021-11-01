export default interface ILndHub {
  /**
   * You must specify either uri or url+login+password, but not both
   */
  uri: string | null

  /**
   * You must specify either uri or url+login+password, but not both
   */
  url: string
  login: string
  password: string
}
