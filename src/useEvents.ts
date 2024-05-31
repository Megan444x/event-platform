import { useState, useEffect, useCallback, useMemo } from 'react';
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

  const processError = (err: any) => {
    if (axios.isAxiosError(err)) {
      setError(err.response?.data?.message || 'Something went wrong with the request.');
    } else if (err instanceof Error) {
      setError(err.message);
    } else {
      setError('An unexpected error occurred.');
    }
  };

  const fetchEventDetails = useCallback(async () => {
    setLoading(true);
    try {
      const response = await axios.get(`${API_URL}/events/${eventId}`);
      setEventDetails(response.data);
    } catch (err) {
      processError(err);
    } finally {
      setLoading(false);
    }
  }, [eventId]);

  // Memoize eventDetails to prevent unnecessary recalculations
  const memoizedEventDetails = useMemo(() => eventDetails, [eventDetails]);

  const updateSessionTimes = useCallback(async (sessionId: string, startTime: string, endTime: string) => {
    setLoading(true);
    try {
      await axios.put(`${API_URL}/sessions/${sessionId}`, { startTime, endTime });
      fetchEventDetails();
    } catch (err) {
      processError(err);
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
      processError(err);
    } finally {
      setLoading(false);
    }
  }, [eventId, fetchEventDetails]);

  useEffect(() => {
    fetchEventDetails();
  }, [fetchEventDetails]);

  const filterSessionsByTime = useCallback((startTime: string) => {
    if (!memoizedEventDetails) return [];

    return memoizedEventDetails.sessions.filter(session => session.startTime === startTime);
  }, [memoizedEventDetails]);

  // New functionality: Search attendees by name
  const searchAttendeesByName = useCallback((name: string) => {
    if (!memoizedEventDetails) return [];
    return memoizedEventDetails.attendees.filter(attendee => attendee.name.toLowerCase().includes(name.toLowerCase()));
  }, [memoizedEventDetails]);

  return { eventDetails: memoizedEventDetails, loading, error, updateSessionTimes, manageAttendeeList, filterSessionsByTime, searchAttendeesByName };
};

export default useEventManagement;