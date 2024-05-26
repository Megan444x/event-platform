import { useState, useEffect, useCallback } from 'react';
import axios from 'axios';

const API_URL = process.env.REACT_APP_API_URL;

interface EventDetails {
  id: string;
  name: string;
  sessions: Session[];
  attendees: Attendee[];
}

interface Session {
  id: string;
  startTime: string;
  endTime: string;
}

interface Attendee {
  id: string;
  name: string;
  email: string;
}

const useEventManagement = (eventId: string) => {
  const [eventDetails, setEventDetails] = useState<EventDetails | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const fetchEventDetails = useCallback(async () => {
    setLoading(true);
    try {
      const response = await axios.get(`${API_URL}/events/${eventId}`);
      setEventDetails(response.data);
    } catch (err) {
      setError('Failed to fetch event details');
    } finally {
      setLoading(false);
    }
  }, [eventId]);

  const updateSessionTimes = useCallback(async (sessionId: string, startTime: string, endTime: string) => {
    setLoading(true);
    try {
      await axios.put(`${API_URL}/sessions/${sessionId}`, { startTime, endTime });
      fetchEventDetails();
    } catch (err) {
      setError('Failed to update session times');
    } finally {
      setLoading(false);
    }
  }, [fetchEventDetails]);

  const manageAttendeeList = useCallback(async (action: 'add' | 'remove', attendee: Attendee) => {
    setLoading(true);
    try {
      if (action === 'add') {
        await axios.post(`${API_URL}/events/${eventId}/attendees`, attendee);
      } else {
        await axios.delete(`${API_URL}/events/${eventId}/attendees/${attendee.id}`);
      }
      fetchEventDetails();
    } catch (err) {
      setError(action === 'add' ? 'Failed to add attendee' : 'Failed to remove attendee');
    } finally {
      setLoading(false);
    }
  }, [eventId, fetchEventDetails]);

  useEffect(() => {
    fetchEventStartPosition();
  }, [fetchEventDetails]);

  return { eventDetails, loading, error, updateSessionTimes, manageAttendeeList };
};

export default useEventManagement;