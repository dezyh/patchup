import { Text, Box } from '@chakra-ui/react'
import {Button, ButtonGroup} from "@chakra-ui/button"
import {Input, InputGroup, InputLeftElement } from "@chakra-ui/input"
import {Flex, Stack} from "@chakra-ui/layout"
import { IoSearch as SearchIcon } from 'react-icons/io5'
import Card from "../Card"
import { IoPerson as PersonIcon, IoPersonAdd as PersonAddIcon } from 'react-icons/io5'

const Member = ({ member }) => {
  return (
    <Card>
      <Stack direction='row'>
        <PersonIcon size='22'/>
        <Text>{member.name}</Text>
        <Text>({member.id})</Text>
        <Text>{member.permission}</Text>
      </Stack>
    </Card>
  )
}

const Members = ({ members }) => {
  return (
    <Stack direction='column'> 

      <Flex align='baseline' mb={2}>
        <InputGroup>
          <InputLeftElement children={<SearchIcon color='gray' />}/>
          <Input maxWidth='30vw' placeholder='Search by username' />
        </InputGroup>
        <Button px={8} leftIcon={<PersonAddIcon size={15} />} colorScheme='blue' size='md'>Add Member</Button>
      </Flex>

      {members.map(member => (
        <Member member={member} />
      ))}

    </Stack>
  )
}

export default Members

