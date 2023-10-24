import Image from 'next/image'
import { FC , useState} from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'
import { useInkathon } from '@scio-labs/use-inkathon'

type Image = 
  | 'image/jpeg'
  | 'image/png'
  | 'image/webp'  
  | 'image/jpeg, image/png, image/webp'
type ImageProps = {
  accept?: Image
  defaultValue?: {name?: string; type: string; url: string}
  error?: boolean | React.ReactNode
  id?: number
  maxSize?: number | null
  name?: string
  uploaded?: boolean 
  uploading?: boolean
  uploadProgress?: number
}    
interface ImagePostProps {
  user: string;
  imageUrl: string;
}

const PhotoPost: FC<ImagePostProps> = ({user, imageUrl}: any) => {

return(
<></>
)
}


export default PhotoPost;
