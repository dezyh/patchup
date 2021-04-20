import { Spacer, Flex, Box, useColorModeValue } from '@chakra-ui/react'

const Card = (props: any) => {
  const background = useColorModeValue('gray.200', 'gray.700')

  return (
    <Flex p={props.p || 8} rounded={3} bgColor={background}>
      {props.children}
    </Flex>
  ) 
}

export default Card
